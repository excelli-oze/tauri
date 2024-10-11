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
class SendDataRequest {
    var message: String? = null
    var number: Int? = null

    override fun toString(): String {
        return "SendDataRequest(message=$message, number=$number)"
    }
}


@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)
        val message = args.value ?: "No message received"
        Log.d("ExamplePlugin", "ping message: $message")

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun sendDataToAndroid(invoke: Invoke) {
        val args = invoke.parseArgs(SendDataRequest::class.java) 
        Log.d("ExamplePlugin", "Received data from JavaScript: $args") 

        val ret = JSObject()
        ret.put("value", "Message received successfully")
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
