import { Client, GatewayIntentBits } from 'discord.js';
import { config } from '../config';
import { commands } from './command';

const client = new Client({ intents: [GatewayIntentBits.Guilds] });

client.on('ready', () => {
    console.log('Ready!');
});

client.on('interactionCreate', (interaction) => {
    if (!interaction.isChatInputCommand()) return;

    commands[interaction.commandName].execute(interaction);
});

client.login(config.token);