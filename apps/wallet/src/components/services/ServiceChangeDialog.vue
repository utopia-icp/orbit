<template>
  <VDialog
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard :loading="submitting">
      <VToolbar color="background">
        <VToolbarTitle> {{ props.app.name }} </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <VCardText>
        <VSelect
          v-if="availableVersions.length"
          v-model="selectedVersion"
          :items="availableVersions"
          :hint="`Checksum is ${wasmModuleChecksum}`"
          :persistent-hint="!!selectedVersion"
          label="Which version would you like to install?"
          outlined
          dense
        />
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn
          :disabled="!selectedVersion || submitting"
          color="primary"
          variant="flat"
          size="small"
          class="ma-4"
          @click="submit"
        >
          Install
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { computed, watch, ref } from 'vue';
import {
  VBtn,
  VCard,
  VCardText,
  VDialog,
  VDivider,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { ServiceInstalled } from '~/types/app.types';
import { arrayBufferToHashHex } from '~/utils/crypto.utils';

const props = withDefaults(
  defineProps<{
    serviceId: string;
    app: ServiceInstalled;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const loading = ref(false);
const submitting = ref(false);
const canClose = computed(() => !loading.value && !submitting.value);
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const selectedVersion = ref<string | null>(null);
const availableVersions = computed(() => props.app.updates.map(update => update.version));
const station = useStationStore();
const wasmModuleChecksum = ref('');
const wasmModule = computed(
  () => props.app.updates.find(update => update.version === selectedVersion.value)?.wasm,
);

function toArrayBuffer(input: Uint8Array | number[]) {
  if (input instanceof Uint8Array) {
    // If the input is already a Uint8Array, use its buffer
    return input.buffer;
  } else if (Array.isArray(input)) {
    // If the input is a number array, create a new Uint8Array from it and then get the buffer
    const uint8Array = new Uint8Array(input);
    return uint8Array.buffer;
  } else {
    throw new TypeError('Input must be a Uint8Array or a number array');
  }
}

watch(
  () => wasmModule.value,
  () => {
    if (!wasmModule.value) {
      wasmModuleChecksum.value = '';
      return;
    }

    const buffer = toArrayBuffer(wasmModule.value);
    arrayBufferToHashHex(buffer).then(checksum => {
      wasmModuleChecksum.value = checksum;
    });
  },
);

const submit = async () => {
  try {
    submitting.value = true;
    if (!wasmModule.value) {
      throw new Error('Module not found');
    }

    const request = await station.service.installService({
      canister_id: Principal.fromText(props.serviceId),
      arg: [],
      mode: { upgrade: null },
      module: wasmModule.value,
    });

    useOnSuccessfulOperation(request);
  } catch (error) {
    logger.error('Failed to install service', error);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
    open.value = false;
    selectedVersion.value = null;
  }
};
</script>
