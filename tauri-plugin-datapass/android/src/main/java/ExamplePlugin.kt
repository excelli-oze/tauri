package com.plugin.datapass

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.util.Log

@InvokeArg
class PingArgs {
    var value: String? = null
}

@InvokeArg
class SendDataArgs {
    var message: String? = null
    var number: Int? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun sendDataToAndroid(invoke: Invoke) {
        val args = invoke.parseArgs(SendDataArgs::class.java)

        val message = args.message ?: "No message provided"
        val number = args.number ?: 0


        val result = "Received message from web to android and passed back to web: '$message' and number: $number"

        val ret = JSObject()
        ret.put("success", true)
        ret.put("message", result)
        invoke.resolve(ret)
    }

     @Command
    fun getDataFromAndroid(invoke: Invoke) {
        // val data = "This message sent from android to web"
        val data = JSObject().apply {
            put("message", "This message sent from android to web")
            put("number", 22)
        }
        val ret = JSObject()
        ret.put("data", data)
        invoke.resolve(ret)
    }
}
