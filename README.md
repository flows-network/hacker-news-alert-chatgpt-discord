# <p align="center">HackerNews Alert</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-the-hackernews-alert-app), and you will reveice HackerNews post alerts per hour according to your interests. 

<img width="658" alt="image" src="https://user-images.githubusercontent.com/45785633/227419393-d7a438f1-51c9-42bc-bb9a-bac1cd3e5581.png">

## Deploy the HackerNews Alert App

To create this App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/hackernews-alert/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying this APP
3. Authenticate the [flows.network](https://flows.network/) to access the `hackernews-alert` repo you just forked. 
![image](https://user-images.githubusercontent.com/45785633/227176033-35a445d8-9e73-4d6d-a919-c68d64cc4075.png)

4. Click on the Advanced text and you will see more settings. Fill in the required Environment Variables. In this example, we have three variables. One is `KEYWORD`: fill in one topic you want to listen to, like `ChatGPT`. The other two variables: `WORKSPACE` and `CHANNEL`: fill in your own workspace and channel

![image](https://user-images.githubusercontent.com/45785633/227176580-b7e8d31d-b871-45b4-baee-312572615e8a.png)

5. At last, click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

![image](https://user-images.githubusercontent.com/45785633/227176699-a1ce1c05-02b9-411a-890f-ece033fde38e.png)

Here we can see, we need to configue one SaaS integration.

Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network` bot on your workspace. This workspace is the one you entered into the environment variables above.

> If you have authenticated the workspace before,you can see the purple Connect button turns gray Connected button. Just ingore this step and click Check button.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the Hackernews alert App goes live. You will get a salck message at the 50th minute of every hour !

![image](https://user-images.githubusercontent.com/45785633/227177456-a51eacda-2f09-4206-874b-4dc73c3408d8.png)

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


