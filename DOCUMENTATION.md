# **Please read this documentation to have the maximum knowledge of what the bot can and what you shouldn't do if you don't want to hang it or trigger its bugs!!!**

## **What you need to have to get the bot working:**
1. You must have computer with **_Windows_** operating system,
2. You must have got installed chrome or ungoogled chromium browswer

## **Why you should encrypted your microsoft teams account login data which is added for bot configuration:**
1. The main reason why you should do this is because of your account security. Your account credentials are stored in a configuration file in the config folder.
If you decide to send someone a bot with unencrypted login data, you will give him access to your account, but if you encrypt it, he will not log into your account because your data will be represented by a string,
which it would take years to decrypt using fo this enormous computing power not available to a single computer unit,
2. Remember that you will encrypt the data only once within 5-10 seconds and their security will be maintained for many years. The decision is yours `yes` or `no` (Would you like encrypt added data (this should prevent your data from being accidentally shared)? - your answer)

## **How use that application:**
1. First you must give your account pass data to enable singin for microsoft teams application using command `teams-automatization.exe config`,
2. Then you can add banned meetings to tell the bot what meetings not to join. To do this, use the following command `teams-automatization.exe config meetings` **or** `teams-automatization.exe config m`,
3. Now you can launch bot using command `teams-automatization.exe run` **or** using flag `teams-automatization.exe -r`,

## **What you can do when the bot is launched:**
1. You can minimalize and maxymalize chrome browswer window and click outside him (you can only do when the bot goes to the calendar),
2. In the meeting you can turn on/off camera, turn on/off microphone, share screen, raise your hand in the microsoft teams app,

## **What you can't do when the bot is launched:**
1. You can't leave the browser window minimizing it or clicking on another window until the bot goes to the calendar,
2. You can't perform any action in the chrome window the bot is running in while it is outside of the meeting,

## **How to update bot configuration**:
>### **How to update the configuration of the ms teams account data provided for the bot?**
1. If you want to udate your microsoft teams account pass data you should use command `teams-automatization.exe config` again!!!
>### **How to update banned meetings setup?**
1. If you would like update your existed banned meeting you should use command `teams-automatization.exe config meetings update` and add banned meeting number from displayed list of banned meetings from your bot configuration [This point will only work if you set any banned meeting in the banned meeting configuration for the bot]
>### **How to add more banned meetings for my application configuration?**
1. If you would like add more banned meetings to your bot banned meetings configurtation you should use command `teams-automatization.exe config meetings` and add meetings data.
2. After giving all the banned meetings that you want in your configuration, you should answer the question "Would you like continue configuration or save it result?" with "sav" in order to save the entered configuration.
3. Then you should answer "no" to the question "Would you like to replace old configuration data by new configuration data?" to add a new meeting setup to the existing setup or answer "yes" to replace the old meeting setup with a new setup 

## **Commands List:**
1. **help/-h/-help/--help** - Use this command to get information about the bot,
2. **config** - This command configures the access data for the application account, which is the password and e-mail for your account,
3. **config m/meetings** - This command configures banned meetings list for your application,
4. **config m dis/display** - This command display banned meetings configured for you application,
5. **config m up/update** - This command updates banned meeting for your application configuration,
6. **config m del/delete** - This command deletes banned meetings from your application configuration,
7. **run/-r** - This command runs your application with setted setup by you,

## **Command Usage Examples: [_windows cmd_]**
1. _teams-automatization.exe_ ***help/--help/-help/-h***
2. _teams-automatization.exe_ ***config***,
3. _teams-automatization.exe_ ***config m/meetings***,
4. _teams-automatization.exe_ ***config m dis/display***,
5. _teams-automatization.exe_ ***config m up/update***,
6. _teams-automatization.exe_ ***config m del/delete***,
7. _teams-automatization.exe_ ***run/-r***

### **Other infos:**
1. You can create shortcut of .exe application and move it in other location then source bot folder but you can't move application .exe file to other location then the bot source folder!!!

![GitHub Dark](/drivers/app_image_icon_oUt_icon.ico)

[Program Image Is Borrowed From This Page](https://artimento.pl/neonowy-kotek)