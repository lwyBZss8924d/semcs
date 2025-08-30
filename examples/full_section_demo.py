#!/usr/bin/env python3
"""
Demo script to show the --full-section feature of ck.
This file contains various Python functions and classes for testing.
"""

import time
from typing import List, Dict, Optional


class DataProcessor:
    """A class that processes various types of data."""
    
    def __init__(self, config: Dict):
        self.config = config
        self.data = []
        self.processed_count = 0
    
    def process_batch(self, items: List[str]) -> List[str]:
        """Process a batch of items with error handling."""
        results = []
        for item in items:
            try:
                # This is where error handling happens
                processed = self._transform(item)
                results.append(processed)
                self.processed_count += 1
            except Exception as e:
                print(f"Error processing {item}: {e}")
                results.append(None)
        return results
    
    def _transform(self, item: str) -> str:
        """Internal transformation logic."""
        return item.upper().replace(" ", "_")


def calculate_statistics(numbers: List[float]) -> Dict[str, float]:
    """Calculate basic statistics for a list of numbers."""
    if not numbers:
        return {"mean": 0, "min": 0, "max": 0}
    
    # Calculate mean, min, max
    mean_value = sum(numbers) / len(numbers)
    min_value = min(numbers)
    max_value = max(numbers)
    
    return {
        "mean": mean_value,
        "min": min_value,
        "max": max_value,
        "count": len(numbers)
    }


async def fetch_data_async(url: str, timeout: int = 30) -> Optional[Dict]:
    """Async function to fetch data from a URL."""
    import asyncio
    
    print(f"Fetching data from {url}")
    # Simulate network delay
    await asyncio.sleep(1)
    
    # Return mock data
    return {
        "url": url,
        "status": "success",
        "data": {"example": "data"},
        "timestamp": time.time()
    }


def retry_with_backoff(func, max_retries: int = 3, delay: float = 1.0):
    """
    Decorator to retry a function with exponential backoff.
    This implements error handling and retry logic.
    """
    def wrapper(*args, **kwargs):
        for attempt in range(max_retries):
            try:
                return func(*args, **kwargs)
            except Exception as e:
                if attempt == max_retries - 1:
                    raise
                wait_time = delay * (2 ** attempt)
                print(f"Attempt {attempt + 1} failed: {e}")
                print(f"Retrying in {wait_time} seconds...")
                time.sleep(wait_time)
    return wrapper


class DatabaseConnection:
    """Manages database connections with pooling."""
    
    def __init__(self, host: str, port: int, db_name: str):
        self.host = host
        self.port = port
        self.db_name = db_name
        self.pool = []
    
    def connect(self):
        """Establish a database connection."""
        # This is where connection logic would go
        connection = f"Connection to {self.host}:{self.port}/{self.db_name}"
        self.pool.append(connection)
        return connection
    
    def execute_query(self, query: str):
        """Execute a database query with error handling."""
        if not self.pool:
            self.connect()
        
        try:
            # Simulate query execution
            print(f"Executing: {query}")
            return {"status": "success", "rows": []}
        except Exception as e:
            print(f"Query failed: {e}")
            raise


if __name__ == "__main__":
    # Example usage
    processor = DataProcessor({"mode": "production"})
    results = processor.process_batch(["hello", "world", "test"])
    print(f"Processed {processor.processed_count} items")
    
    stats = calculate_statistics([1.5, 2.7, 3.9, 4.2, 5.1])
    print(f"Statistics: {stats}")