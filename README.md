# <p align="center">Summarize Hacker News Posts Using ChatGPT</p>
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

[Deploy this function on flows.network](#deploy-your-own-hacker-news-summary-bot-in-3steps), and you will receive HackerNews post alerts per hour according to your interests. More importantly, the bot will summarize the Hacker News post with the power of ChatGPT.

![image](https://github.com/flows-network/hacker-news-alert-chatgpt-discord/assets/45785633/77463fb6-ffa5-4d15-b032-0549b9146786)


## How it works

This is a scheduled hourly bot, meaning it is triggered based on the set time. When the specified time arrives, the bot will search for all the Hacker News posts in the past hour and identify those that contain the specified keyword. Subsequently, the bot will send you a Discord message and utilize ChatGPT to provide a summary of the posts.

## Deploy your own Hacker News summary bot in 3 steps

1. Create a bot from a template
2. Add your OpenAI API key
3. Configure the bot on a specified Discord channel

### 0 Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Create a bot from a template


Load [the Hacker News Alert ChatGPT Discord template](https://flows.network/flow/createByTemplate/hacker-news-alert-chatgpt-discord).

Review the `KEYWORD` variable. Type the keyword you're concerned about. Only support one Keyword here.

Click on the **Create and Build** button.

### 2 Add your OpenAI API key

You will now set up OpenAI integration. Click on **Connect**, and enter your key. The default key name is `Default`.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click on **Continue**.

### 3 Configure the bot to access Discord

You will now set up the Discord integration. Enter the `discord_channel_id` and `discord_token` to configure the bot. [Click here to learn how to get a Discord channel id and Discord bot token](https://flows.network/blog/discord-bot-guide).

* `discord_channel_id`: specify the channel where you wish to deploy the bot. You can copy and paste the final set of serial numbers from the URL.
* `discord_token`: get the Discord token from the Discord Developer Portal. This is standalone.

<img width="658" alt="image" src="https://github.com/flows-network/hacker-news-alert-chatgpt-discord/assets/45785633/1af8d30c-89b2-4771-96a2-68c0e9bee3c3">

Finally, click on **Deploy**.

## Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to summarize the Hacker News Post. 


## FAQ

### How to customize the time when the bot sends Discord messages

To customize the time when the bot sends Discord messages, you can modify the value provided within the cron expression ("37 * * * *"). This expression represents the bot sending messages at the 37th minute of every hour.

```
    schedule_cron_job(String::from("37 * * * *"), keyword, callback).await;
```

To adjust the timing, you can change the number 37 to your desired minute. For example, if you want the messages to be sent at the 15th minute of every hour, you can modify the expression to be ("15 * * * *").

Remember to use the appropriate format and values for the minutes (0-59), hours (0-23), days of the month (1-31), months (1-12), and days of the week (0-7, where both 0 and 7 represent Sunday).

By customizing the cron expression accordingly, you can set the desired timing for the bot to send Discord messages.








