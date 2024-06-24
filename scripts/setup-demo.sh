#!/usr/bin/env bash
set -eEuo pipefail

NETWORK="production"

function setup_enviroment() {
  . ./scripts/setup-node.sh

  if ! command -v pnpm >/dev/null 2>&1; then
    echo "pnpm not found, installing..."
    npm install -g pnpm
  fi

  pnpm install
}

function build_wasms() {
  echo "Building the WASMs for the station and upgrader canisters."

  ./scripts/generate-wasm.sh station
  ./scripts/generate-wasm.sh upgrader
}

function deploy_control_panel() {
  echo "Deploying the control_panel canister to the '$NETWORK' network."

  echo "Building the control_panel wasm..."

  ./scripts/generate-wasm.sh control-panel

  # Read the WASM files and convert them to hex format
  upgrader_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./wasms/upgrader.wasm.gz | sed 's/../\\&/g')
  station_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./wasms/station.wasm.gz | sed 's/../\\&/g')

  set +e # Disable 'exit on error'
  canister_id_output=$(dfx canister id control_panel --network $NETWORK 2>&1)
  canister_id_exit_code=$?
  set -e # Re-enable 'exit on error'

  if [ $canister_id_exit_code -ne 0 ]; then
    echo "Canister 'control_panel' does not exist, creating and installing..."

    dfx canister create control_panel --network $NETWORK --with-cycles 10000000000000
    dfx canister install control_panel --network $NETWORK --wasm ./wasms/control_panel.wasm.gz
  else
    echo "Canister 'control_panel' already exists with ID: $canister_id_output"

    module_hash=$(dfx canister info control_panel --network $NETWORK | grep "Module hash" | awk '{print $3}')

    if [ "$module_hash" == "None" ]; then
      echo "Installing the wasm module to the control_panel canister..."
      dfx canister install control_panel --network $NETWORK --wasm ./wasms/control_panel.wasm.gz --mode install
    else
      echo "Upgrading the wasm module to the control_panel canister..."
      dfx canister install control_panel --network $NETWORK --wasm ./wasms/control_panel.wasm.gz --mode upgrade --yes
    fi
  fi

  echo "Updating the control_panel canister with the new station and upgrader WASM modules..."
  dfx canister call control_panel --network $NETWORK upload_canister_modules --argument-file <(echo "(record { upgrader_wasm_module = blob \"$upgrader_wasm_module_bytes\"; station_wasm_module = blob \"$station_wasm_module_bytes\"; })")
}

function deploy_app_wallet() {
  echo "Deploying the Orbit Wallet to the '$NETWORK' network."

  BUILD_MODE=$NETWORK dfx deploy --network $NETWORK app_wallet --with-cycles 2000000000000
}

function add_apps_to_registry() {
  echo "Adding the demo wasms to the control-panel registry under @demo namespace."

  # Read the WASM files and convert them to hex format
  v1_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./app-store/unstoppable-app-v1.wasm.gz | sed 's/../\\&/g')
  v2_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./app-store/unstoppable-app-v2.wasm.gz | sed 's/../\\&/g')

  # Add the first version of the unstoppable app to the registry
  dfx canister call control_panel --network $NETWORK add_registry_entry --argument-file <(echo "(
    record { 
      entry = record {
        name = \"@demo/unstoppable-app\";
        description = \"An unstoppable app running in the UTOPIA private compute network.\";
        tags = vec { \"unstoppable\"; \"app\" };
        categories = vec { \"app\" };
        metadata = vec {};
        value = variant {
          WasmModule = record {
            wasm_module = blob \"$v1_wasm_module_bytes\";
            version = \"1.0.0\";
            dependencies = vec {};
          }
        }
      }
    }
  )")

  # Add the second version of the unstoppable app to the registry
  dfx canister call control_panel --network $NETWORK add_registry_entry --argument-file <(echo "(
    record { 
      entry = record {
        name = \"@demo/unstoppable-app\";
        description = \"An unstoppable app running in the UTOPIA private compute network.\";
        tags = vec { \"unstoppable\"; \"app\" };
        categories = vec { \"app\" };
        metadata = vec {};
        value = variant {
          WasmModule = record {
            wasm_module = blob \"$v2_wasm_module_bytes\";
            version = \"2.0.0\";
            dependencies = vec {};
          }
        }
      }
    }
  )")
}

# can be used to delete a registry entry by id
function delete_registry_entry() {
  entry_id=$1

  dfx canister call control_panel --network $NETWORK delete_registry_entry "(record { id = \"$entry_id\" })"
}

# Run the setup functions in order to create the demoable UTOPIA
setup_enviroment
# build_wasms
# deploy_control_panel
deploy_app_wallet
# add_apps_to_registry
