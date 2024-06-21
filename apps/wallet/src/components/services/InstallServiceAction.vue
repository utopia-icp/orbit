<template>
  <VBtn
    v-bind="$attrs"
    :size="props.size"
    :variant="props.variant"
    :icon="props.icon && !props.text"
    :color="props.color"
    @click="open = true"
  >
    <VIcon v-if="props.icon" class="mr-1" :icon="props.icon" />
    <slot name="default">
      <span v-if="props.text">{{ props.text }}</span>
    </slot>
    <VIcon v-if="props.appendIcon" class="ml-1" :icon="props.appendIcon" />
  </VBtn>

  <InstallServiceDialog
    :service-id="props.serviceId"
    :apps="props.apps"
    :open="open"
    :readonly="props.readonly"
    :dialog-max-width="800"
    @update:open="
      openEvent => {
        open = openEvent;

        emit('opened', openEvent);
      }
    "
  />
</template>
<script lang="ts" setup>
import { ref } from 'vue';
import { VBtn } from 'vuetify/components';
import InstallServiceDialog from './InstallServiceDialog.vue';
import { RegistryApp } from '~/types/app.types';

const props = withDefaults(
  defineProps<{
    serviceId: string;
    apps: RegistryApp[];
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'tonal' | 'elevated';
    color?: string;
    readonly?: boolean;
    appendIcon?: string;
  }>(),
  {
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'primary',
    readonly: false,
    appendIcon: undefined,
  },
);

const open = ref(false);

const emit = defineEmits<{
  (event: 'opened', payload: boolean): void;
}>();
</script>
