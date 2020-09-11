import collections

# from random import choice
import random

Card = collections.namedtuple("Card", ["rank", "suit"])


class FrenchDeck:
    ranks = [str(n) for n in range(2, 11)] + list("JQKA")
    suits = "spades diamonds clubs hearts".split()

    def __init__(self):
        self._cards = [Card(rank, suit) for suit in self.suits for rank in self.ranks]

    def __len__(self):
        return len(self._cards)

    def __getitem__(self, position):
        return self._cards[position]

    def shuffle(self):
        random.shuffle(self._cards)

    def print_deck(self):
        for card in self._cards:
            print(card.rank, card.suit)


if __name__ == "__main__":
    theDeck = FrenchDeck()
    print("unshuffled")
    theDeck.print_deck()
    # 	print(choice(theDeck).rank)
    theDeck.shuffle()
    print("shuffled")
    theDeck.print_deck()
