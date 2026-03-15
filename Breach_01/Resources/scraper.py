import requests
from bs4 import BeautifulSoup

BASE_URL = "http://localhost:8080/.hidden/"

# the common messages to ignore
TROLL_MESSAGES = [
    "Non ce n'est toujours pas bon ...",
    "Tu veux de l'aide ? Moi aussi !",
    "Demande à ton voisin du dessous",
    "Demande à ton voisin de gauche",
    "Toujours pas tu vas craquer non ?",
    "Demande à ton voisin de droite",
    "Demande à ton voisin du dessus",
]

visited = set()


def crawl(url) -> None:
    if url in visited:
        return
    visited.add(url)

    try:
        response = requests.get(url)
        soup = BeautifulSoup(response.text, "html.parser")
        for link in soup.find_all("a"):
            href = link.get("href")

            # skip parent directory links
            if href == "../":
                continue

            # build url
            full_url = url + href
            if href == "README":
                content = requests.get(full_url).text.strip()
                if content not in TROLL_MESSAGES:
                    print(f"Content: {content}\n")
                else:
                    print(".", end="", flush=True)
            elif href.endswith("/"):
                crawl(full_url)

    except Exception as e:
        print(f"Error accessing {url}: {e}")


if __name__ == "__main__":
    crawl(BASE_URL)
