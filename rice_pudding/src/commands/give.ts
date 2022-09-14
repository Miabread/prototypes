import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';

export const give: Command = {
    builder: new SlashCommandBuilder()
        .setDescription('Give a rice pudding to someone')
        .addUserOption((option) =>
            option
                .setName('person')
                .setDescription('Person to give a rice pudding to')
                .setRequired(true),
        )
        .addNumberOption((option) =>
            option
                .setName('amount')
                .setDescription('Amount of pudding to give')
                .setMinValue(1)
                .setMaxValue(256)
                .setRequired(true),
        ),

    async execute(interaction, prisma) {
        const sender = interaction.user;
        const senderData = await prisma.user.upsert({
            where: { id: sender.id },
            update: { username: sender.username },
            create: { id: sender.id, username: sender.username },
        });

        const amount = interaction.options.getNumber('amount', true) ?? 1;

        if (senderData.pudding < amount) {
            return interaction.reply(
                `You only have ${senderData.pudding} rice pudding!`,
            );
        }

        const receiver = interaction.options.getUser('person', true);
        const receiverData = await prisma.user.upsert({
            where: { id: receiver.id },
            update: { username: receiver.username },
            create: { id: receiver.id, username: receiver.username },
        });

        await prisma.$transaction([
            prisma.user.update({
                where: { id: sender.id },
                data: { pudding: senderData.pudding - amount },
            }),
            prisma.user.update({
                where: { id: receiver.id },
                data: { pudding: receiverData.pudding + amount },
            }),
        ]);

        const content = `You gave <@${
            receiver.id
        }> ${amount} rice pudding! You have ${
            senderData.pudding - amount
        } pudding left.`;

        interaction.reply({
            content,
            allowedMentions: {
                users: [receiver.id],
            },
        });
    },
};
