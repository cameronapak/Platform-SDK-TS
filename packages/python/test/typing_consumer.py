from cameronapak_platform_sdk import AsyncPlatformClient, PlatformClient
from cameronapak_platform_sdk.highlights import V1HighlightsCollectionPostRequestHighlight

sync = PlatformClient(yvp_app_key="app")
async_client = AsyncPlatformClient(yvp_app_key="app", token="bearer")

page = sync.bibles.collection_get(language_ranges=["en", "es"], page_size=17)
if page is not None:
    next_page: str | None = page.next_page_token


async def examples() -> None:
    await async_client.search_topics.v1search_topics_collection_get(query="hope", language_ranges=["en"])
    await async_client.search_queries.v1search_queries_collection_get(query="hope", language_ranges=["en"])
    await async_client.search_unified.v1search_unified_collection_get(query="hope", bible_id=3034, language_ranges=["en"])


sync.highlights.v1highlights_collection_post(
    request_id="request-id",
    highlight=V1HighlightsCollectionPostRequestHighlight(
        bible_id=3034, passage_id="MAT.1.1", color="44aa44"
    ),
)

# These calls intentionally prove that required language ranges and numeric IDs are checked.
sync.bibles.collection_get()  # type: ignore[call-arg]
sync.search_queries.v1search_queries_collection_get(query="hope")  # type: ignore[call-arg]
sync.search_topics.v1search_topics_collection_get(query="hope")  # type: ignore[call-arg]
sync.search_unified.v1search_unified_collection_get(query="hope", bible_id=3034)  # type: ignore[call-arg]
sync.bibles.resource_get("not-an-integer")  # type: ignore[arg-type]
