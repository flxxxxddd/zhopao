import { Bot } from "gramio";
import "dotenv/config";

const jopaaPhrases = [
  "ЖОПА",
  "жопо?",
  "жопи-жопи",
  "ЖО-ПАА!",
];

const bot = new Bot(process.env.BOT_TOKEN);

bot.on("inline_query", async (context) => {
  const randomPhrase = phrases[Math.floor(Math.random() * jopaaPhrases.length)];

  await context.answerInlineQuery([
    {
      type: "article",
      id: "1",
      title: "жопа",
      input_message_content: {
        message_text: randomPhrase,
      },
    },
  ]);
});

bot.start()

