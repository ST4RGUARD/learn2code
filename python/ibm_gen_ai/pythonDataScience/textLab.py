# %%
import re


class TextUtils:
    def __init__(self):
        pass

    def lowercase(self, words):
        return words.lower()

    def wordFrequency(self, sentence):
        words = sentence.split()
        word_freq = {}
        for word in words:
            freq = re.findall(word, sentence)
            word_freq[word] = freq
        return word_freq

    def countWord(self, sentence, word):
        return len(re.findall(word, sentence))


# %%

words = "Blahoiwj toijsjo IS jfowjfoi STOUP this is a test, i know what you mean it is testing me what do you think is it a test . i don't know how many it is test test ok haha"
utils = TextUtils()
sentence = utils.lowercase(words)
print(f"original customer feedback: {sentence}")
word_freq = utils.wordFrequency(sentence)
print("here is a breakdown of each occurence of the words in the customer feedback")
for key, value in word_freq.items():
    print(f"{key}: {value}")

specific_word = "test"
word_count = utils.countWord(sentence, specific_word)
print(f"there are {word_count} occurences of")
