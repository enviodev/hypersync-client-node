import { HypersyncClient } from "@envio-dev/hypersync-client";

async function main() {
    // Create hypersync client using the mainnet hypersync endpoint
    const client = new HypersyncClient({
        url: "https://eth.hypersync.xyz",
        apiToken: process.env.ENVIO_API_TOKEN!,
    });

    console.log("Starting height stream...");

    // Create a height stream to monitor blockchain height changes
    const heightStream = await client.streamHeight();

    console.log("Height stream created. Listening for height updates...");

    // Track the last known height to detect changes

    try {
        while (true) {
            // Receive the next event from the height stream
            const event = await heightStream.recv();

            // Handle different types of events
            switch (event.type) {
                case "Height":
                    console.log(`Height: ${event.height}`);
                    break;

                case "Connected":
                    console.log(`Connected to height stream`);
                    break;

                case "Reconnecting":
                    console.log(
                        `Reconnecting to height stream in ${event.delayMillis}ms due to error: ${event.errorMsg}`,
                    );
                    break;

                default:
                    // Tells the typescript compiler that we have covered all possible event types
                    const _exhaustiveCheck: never = event;
                    throw new Error("Unhandled event type");
            }
        }
    } catch (error) {
        console.error("Error in height stream:", error);
    } finally {
        // Always close the stream to clean up resources
        await heightStream.close();
        console.log("Height stream closed");
    }
}

main().catch(console.error);
