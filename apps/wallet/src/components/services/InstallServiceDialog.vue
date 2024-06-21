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
        <VToolbarTitle> Configuration </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />
      <VCardText>
        <VSelect
          v-model="selectedApp"
          :items="availableApps"
          label="What service would you like to install?"
          outlined
          dense
        />
        <VSelect
          v-if="selectedApp && availableVersions.length"
          v-model="selectedVersion"
          :items="availableVersions"
          label="Which version would you like to install?"
          outlined
          dense
        />
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn
          :disabled="!selectedApp || !selectedVersion || submitting"
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
import { computed, ref } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSelect,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { RegistryApp } from '~/types/app.types';

const props = withDefaults(
  defineProps<{
    serviceId: string;
    apps: RegistryApp[];
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

const submitting = ref(false);
const canClose = computed(() => !submitting.value);
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const selectedApp = ref<string | null>(null);
const availableApps = computed(() => props.apps.map(app => app.name));
const selectedVersion = ref<string | null>(null);
const availableVersions = computed(() => {
  const app = props.apps.find(app => app.name === selectedApp.value);

  return app ? app.versions.map(version => version.version) : [];
});

const station = useStationStore();

const submit = async () => {
  try {
    submitting.value = true;
    const module = props.apps
      .find(app => app.name === selectedApp.value)
      ?.versions.find(version => version.version === selectedVersion.value)?.wasm;

    if (!module) {
      throw new Error('Module not found');
    }

    const request = await station.service.installService({
      canister_id: Principal.fromText(props.serviceId),
      arg: [],
      mode: { install: null },
      module,
    });

    useOnSuccessfulOperation(request);
  } catch (error) {
    logger.error(`Failed to submit account ${error}`);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
    open.value = false;
  }
};
</script>
