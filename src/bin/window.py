from typing import Generic, Iterable, Self, TypeVar
from collections import deque


T = TypeVar("T")


class Window(Generic[T]):
    def __init__(
        self, iterable: Iterable[T], window_size: int, strict: bool = False
    ) -> None:
        self.window_size = window_size
        self.iterator = iter(iterable)
        self.curr_window = deque()
        self.done = False
        self.strict = strict
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
            if self.strict:
                raise ValueError(
                    "strict mode: window size does not divide iterator length"
                )
            self.done = True
        self.curr_window.popleft()
        return ret
