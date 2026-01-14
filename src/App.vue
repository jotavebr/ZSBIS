<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { supabase } from "./lib/supabase";
import { onMounted } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";

const nameConfig = ref("");
const alertMessage = ref("");
const enableAlert = ref(false);

/**
 * Lógica para SALVAR:
 * Rust lê o arquivo -> Vue transforma em JSON -> Supabase salva na tabela
 */
async function sendConfigsToDB() {
    if (!nameConfig.value) {
        alertMessage.value =
            "Para poder salvar, por favor insira um nome para a configuração.";
        enableAlert.value = true;
        return;
    }

    try {
        const jsonContent = await invoke("read_zed_settings");

        const { error } = await supabase.from("user_configs").upsert(
            {
                config_name: nameConfig.value.trim(),
                settings_content: jsonContent,
            },
            { onConflict: "config_name" },
        );

        // Limpa o input após enviar para o DB
        nameConfig.value = "";

        if (error) throw error;
        alertMessage.value = "Configuração salvada com sucesso!";
        enableAlert.value = true;
        console.log("Configuração limpa e salva com sucesso!");
    } catch (err: any) {
        alertMessage.value = err.message;
        enableAlert.value = true;
        console.error("Erro detalhado:", err);
    }
}

/**\
 * Lógica para IMPORTAR:
 * Supabase busca JSON -> Vue transforma em String -> Rust escreve no arquivo
 */
async function getConfigsToDB() {
    if (!nameConfig.value) {
        alertMessage.value =
            "Para importar uma configuração, por favor insira o nome desejado.";
        enableAlert.value = true;
        return;
    }

    try {
        // 1. Busca os dados
        const { data, error, status } = await supabase
            .from("user_configs")
            .select("settings_content")
            .eq("config_name", nameConfig.value.trim())
            .single();

        if (error) {
            console.error("Erro Supabase:", error);
            alertMessage.value = error.message;
            enableAlert.value = true;
            throw new Error(`Erro ${status}: ${error.message}`);
        }

        if (!data) throw new Error("Configuração não encontrada.");

        // 2. Transforma em String
        const contentString = JSON.stringify(data.settings_content, null, 2);

        // 3. Chama o Rust (Verifique se registrou 'write_zed_settings' no main.rs/lib.rs!)
        await invoke("write_zed_settings", { content: contentString });

        // Limpa o input após importar do DB
        nameConfig.value = "";
    } catch (err: any) {
        alertMessage.value = err.message;
        enableAlert.value = true;
        console.error("Falha total:", err);
    }
}

function closeAlertWindow() {
    enableAlert.value = false;
    alertMessage.value = "";
}

async function openSettingsList() {
    await openUrl("https://www.google.com");
}

// Desabilitar o context menu
onMounted(() => {
    document.addEventListener(
        "contextmenu",
        (e) => {
            e.preventDefault();
        },
        false,
    );
});
</script>

<template>
    <main class="main-container">
        <form @submit.prevent="sendConfigsToDB" class="container">
            <input
                class="input-action"
                v-model="nameConfig"
                type="text"
                placeholder="Settings name from DB"
            />
            <div class="buttons-container">
                <button
                    type="submit"
                    @click="sendConfigsToDB"
                    class="btn-action"
                >
                    Save Settings
                </button>
                <button
                    type="button"
                    @click="getConfigsToDB"
                    class="btn-action"
                >
                    Import settings
                </button>
            </div>
        </form>
        <button class="link-action" @click="openSettingsList">Open DB</button>
    </main>
    <div class="alert-messages" :class="{ on: enableAlert }">
        <div class="center">
            <span class="message">{{ alertMessage }}</span>
        </div>
        <div class="bottom">
            <button class="btn-action" @click="closeAlertWindow">
                Close error window
            </button>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "sass:color";

$button-bg: rgb(66, 66, 66);
$button-hover-font-color: rgb(255, 150, 150);
$dark-shadow-1: rgba(0, 0, 0, 0.5);

.main-container {
    padding: 15px;
    display: flex;
    flex-direction: column;
    gap: 15px;
    user-select: none;

    > .container {
        display: flex;
        flex-direction: column;
        gap: 15px;

        > .input-action {
            width: 100%;
            height: 35px;
            font-size: 1rem;
            font-weight: 600;
            background-color: $button-bg;
            border: none;
            border-radius: 10px;
            box-shadow: 3px 3px 5px $dark-shadow-1;
            box-sizing: border-box;
            outline: none;
            padding: 10px;
            cursor: text;

            &:hover {
                background-color: color.adjust($button-bg, $lightness: -5%);
            }

            &:active,
            &:focus {
                background-color: color.adjust($button-bg, $lightness: -10%);

                &::placeholder {
                    color: color.adjust(
                        $button-hover-font-color,
                        $lightness: -10%
                    );
                }
            }
        }

        > .buttons-container {
            display: flex;
            flex-direction: row;
            gap: 10px;
        }
    }
}

.btn-action,
.link-action {
    user-select: none;
    width: 100%;
    height: 35px;
    display: flex;
    flex-direction: column;
    font-size: 1rem;
    font-weight: 600;
    justify-content: center;
    text-decoration: none;
    text-align: center;
    background-color: $button-bg;
    border: none;
    border-radius: 10px;
    box-shadow: 3px 3px 5px $dark-shadow-1;
    box-sizing: border-box;
    outline: none;

    cursor: pointer;

    &:hover {
        background-color: color.adjust($button-bg, $lightness: -5%);
        color: $button-hover-font-color;
    }

    &:active {
        background-color: color.adjust($button-bg, $lightness: -10%);
        color: color.adjust($button-hover-font-color, $lightness: -10%);
    }
}

.alert-messages {
    user-select: none;
    position: fixed;
    top: 0;
    left: 0;
    background-color: rgb(0, 0, 0, 0.8);
    display: none;
    width: 100%;
    height: 100%;
    padding: 10px;
    box-sizing: border-box;
    font-size: 1.2rem;
    text-wrap: wrap;
    text-align: center;
    font-weight: 400;
}

.alert-messages.on {
    display: grid;
    grid-template-rows: 1fr 40px;
}
</style>
