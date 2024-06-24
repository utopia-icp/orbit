<template>
  <div class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>Service ID</template>
      <template #content>
        {{ props.operation.canister_id.toText() }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>Mode</template>
      <template #content>
        {{ mode }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>Checksum</template>
      <template #content>
        {{ props.operation.module_checksum }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="props.operation.arg_checksum?.[0]">
      <template #name>{{ $t('terms.arg') }}</template>
      <template #content>
        {{ props.operation.arg_checksum[0] }}
      </template>
    </RequestOperationListRow>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { ChangeExternalCanisterOperation, Request } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: ChangeExternalCanisterOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const mode = computed(() => {
  if (variantIs(props.operation.mode, 'install')) {
    return 'Install';
  }

  if (variantIs(props.operation.mode, 'reinstall')) {
    return 'Reinstall';
  }

  return 'upgrade';
});
</script>
