import asyncio
import temporalio
import temporalio.client

async def main():
    # Test connection
    client = await temporalio.client.Client.connect("localhost:7233")
    print(f'{client.namespace=}')


if __name__ == "__main__":
    asyncio.run(main())
