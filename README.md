# <p align="center">Summarize Hacker News Posts Using ChatGPT -- Discord version</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/hacker-news-alert-chatgpt-discord">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-your-own-hacker-news-summary-bot-in-3-steps) and receive hourly alerts with summarized Hacker News posts tailored to your interests.

![image](https://github.com/flows-network/hacker-news-alert-chatgpt-discord/assets/45785633/77463fb6-ffa5-4d15-b032-0549b9146786)

> You can also [send the ChatGPT summary as a Slack Message](https://github.com/flows-network/hacker-news-alert-chatgpt-slack).
## How it works

This scheduled bot uses ChatGPT to summarize Hacker News posts. At the specified time, the bot searches for posts from the past hour, filters them based on your chosen keyword, and sends you a Discord message with a summary.

## Deploy your own Hacker News summary bot in 3 steps

1. Create a bot from a template
2. Add your OpenAI API key
3. Configure the bot on a specified Discord channel

### 0 Prerequisites

* You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

* Sign up on [flows.network](https://flows.network/) using your GitHub account. It is free.

### 1 Create a bot from a template


Go to [the Hacker News Alert ChatGPT Discord template](https://flows.network/flow/createByTemplate/hacker-news-alert-chatgpt-discord).

Review the `KEYWORD` variable to specify your keyword of interest (supporting only one keyword for each bot).

Click on the **Create and Build** button.

### 2 Configure the bot to access Discord

Set up the Discord integration. Enter the `discord_channel_id` and `discord_token` to configure the bot. [Click here to learn how to get a Discord channel id and Discord bot token](https://flows.network/blog/discord-bot-guide).

* `discord_channel_id`: Specify the channel where you want to deploy the bot. Copy and paste the final set of serial numbers from the discord channel's webpage URL.
* `discord_token`: Get the Discord token from the Discord Developer Portal. 

<img width="658" alt="image" src="https://github.com/flows-network/hacker-news-alert-chatgpt-discord/assets/45785633/1af8d30c-89b2-4771-96a2-68c0e9bee3c3">

Click **Continue**.

### 2 Add your OpenAI API key

Set up the OpenAI integration. Click on **Connect**, and enter your key. The default key name is `Default`.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Finally, click **Deploy**.

## Wait for the magic!

You are now on the flow details page and the flow function takes a few seconds to build. Once the flow's status changes to `running`, your bot is ready to summarize Hacker News posts.


## FAQ

### How to customize the bot's scheduled messaging time?

To customize the time when the bot sends Discord messages, you can modify the value in the cron expression ("37 * * * *"). This expression means the bot sends messages at the 37th minute of every hour.

```
    schedule_cron_job(String::from("37 * * * *"), keyword, callback).await;
```

To adjust the timing, you can change the number 37 to your desired minute. For example, if you want the messages to be sent at the 15th minute of every hour, you can modify the expression to be ("15 * * * *").

By customizing the cron expression, you can set the desired timing for the bot to send Discord messages.








