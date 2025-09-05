Prerequisites:

(Thanks to timekeeper.top for introducing me to Fiddler!)
1. Download Fiddler (Classic is enough): https://www.telerik.com/download/fiddler
2. “Tools” → “Options” → “HTTPS” and select “Decrypt HPPS Traffic” and accept certificate. You can change "...from all processes" to "from non-browsers" if you want to make the next step easier.
<img width="564" height="369" alt="Fiddler-Decrypt-HTTPS" src="https://github.com/user-attachments/assets/094f23f6-283e-4384-9d54-584d71154599" />

3. With Fiddler running, open the "history" tab on any Contract screen in P5X.
4. Use Ctrl+F to find a URL including "gacha/getRecords" and copy it (note that the following image is taken from an equivalent guide for Reverse 1999).
<img width="987" height="448" alt="Link-In-Fiddler" src="https://github.com/user-attachments/assets/4f8ccd66-add8-4a22-a01d-a0db6e5a537b" />

5. Run p5xsummons.exe: https://github.com/mpvio/p5xsummons/releases/tag/1.0.0
<img width="302" height="132" alt="image" src="https://github.com/user-attachments/assets/57b6a97a-ca13-4d91-b2b0-ba376c73f14b" />

6. Paste the URL in the textbox and press Submit.
7. Once the UI changes, there will be a "p5xsummons.json" and an "authKey.txt" in the same folder as the .exe.
8. The former contains all summon data available on the server, while the latter is your authentication key for accessing said information.
9. The next time you use the app (and as long as authKey.txt is in your directory) you can skip using Fiddler and just press Submit. You don't need P5X running to do so either, though the authKey will expire after ~12 hours.
10. Enjoy! If you're using this app when it first released (early September 2025) the game should still be young enough for you to grab all your summon data from launch.

TODO:
1. Other summon trackers store your data locally too, but I'd like to host a website for easier access to this code than running an .exe and holding onto a .json file.
2. Character/ Weapon/ Personas are note named in the .json file as I am yet to find a database (or even just another map.json) showing which item ID belongs to which item. I'll update this code once I do. (If you know more about P5X, please get in touch!)

FINALLY:
* Fiddler instructions adapted from: https://www.timekeeper.top/auto_import.html
* If you found this useful, please consider donating to my ko-fi: https://ko-fi.com/theroonco
