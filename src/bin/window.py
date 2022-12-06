from typing import Collection, Generic, Self, TypeVar
from collections import deque


T = TypeVar("T")


class Window(Generic[T]):
    def __init__(self, iterable: Collection[T], window_size: int) -> None:
        self.window_size = window_size
        self.iterator = iter(iterable)
        self.curr_window = deque()
        self.done = False
        for _ in range(window_size):
            try:
                self.curr_window.append(next(self.iterator))
            except StopIteration:
                raise ValueError("iterator length is less than window size")

    def __iter__(self) -> Self:
        return self

    def __next__(self) -> tuple[T]:
        if self.done:
            raise StopIteration
        ret = tuple(self.curr_window)
        try:
            self.curr_window.append(next(self.iterator))
        except StopIteration:
            self.done = True
        self.curr_window.popleft()
        return ret
