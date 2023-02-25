<script setup>
import TheWelcome from '../components/TheWelcome.vue'

import {AdminPanelClient} from "../lib/trpc/proto/admin.client";
import { IndexListRequest } from "../lib/trpc/proto/index";
import {GrpcTransport} from '../lib/trpc/transport';

const admin = new AdminPanelClient(GrpcTransport);

console.log(admin.getIndexes(IndexListRequest.create()));

</script>

<template>
  <main>
    <TheWelcome />
  </main>
</template>
