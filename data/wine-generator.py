import json
import random

#
# A random winelist generator
#

# example fields
wines_data = {
    "USA": {
        "provinces": ["California", "Oregon", "Washington"],
        "titles": ["Chardonnay", "Merlot", "Zinfandel"],
        "wineries": ["Napa Valley Winery", "Robert Mondavi", "Ridge Vineyards"]
    },
    "France": {
        "provinces": ["Bordeaux", "Burgundy", "Rhone"],
        "titles": ["Cabernet Sauvignon", "Pinot Noir", "Syrah"],
        "wineries": ["Chateau Margaux", "Domaine de la Romanee-Conti", "E. Guigal"]
    },
    "Italy": {
        "provinces": ["Tuscany", "Piedmont", "Veneto"],
        "titles": ["Sangiovese", "Nebbiolo", "Prosecco"],
        "wineries": ["Antinori", "Gaja", "Masi"]
    },
    "Argentina": {
        "provinces": ["Mendoza", "Salta", "Patagonia"],
        "titles": ["Malbec", "Torrontes", "Bonarda"],
        "wineries": ["Catena Zapata", "Bodega Colome", "Bodega del Fin del Mundo"]
    },
    "Germany": {
        "provinces": ["Rheingau", "Mosel", "Pfalz"],
        "titles": ["Riesling", "Spatburgunder", "Silvaner"],
        "wineries": ["Schloss Johannisberg", "Dr. Loosen", "Bassermann-Jordan"]
    },
    "Spain": {
        "provinces": ["Rioja", "Ribera del Duero", "Priorat"],
        "titles": ["Tempranillo", "Garnacha", "Monastrell"],
        "wineries": ["Marques de Riscal", "Vega Sicilia", "Clos Mogador"]
    },
    "Australia": {
        "provinces": ["Barossa Valley", "Hunter Valley", "Margaret River"],
        "titles": ["Shiraz", "Chardonnay", "Cabernet Sauvignon"],
        "wineries": ["Penfolds", "Tyrell's Wines", "Vasse Felix"]
    }
}

# phrases for description
phrases_intro = [
    "Chapoutier's selections of the best parcels of vines in",
    "Sold under the ancient spelling of the appellation name,",
    "They represent the epitome of the power and concentration that lies behind the reputation of",
    "This cuvée is the best of the collection, with its brooding, opaque character,",
    "Age it until your new-born baby is old enough to drink, and it will be just about ready.",
    "The finest expressions of terroir are found in",
    "Crafted with precision and care, the vineyards of",
    "A testament to the winemaker's art, the wines of",
    "The vineyards produce exceptional quality in",
    "Renowned for their elegance and depth, the wines of",
    "Celebrated for their distinctive flavors, the wines of",
    "From the heart of",
    "The region's winemaking heritage shines through in",
    "Esteemed for their complexity, the wines of",
    "The craftsmanship behind these wines is unparalleled in",
    "These wines offer a unique glimpse into the terroir of",
    "Hailing from the prestigious vineyards of",
    "These selections are a benchmark for excellence in",
    "Drawing from the rich soil of",
    "Each bottle from"
]

phrases_middle = [
    "are set to become legendary.",
    "suggesting rather than revealing power at this stage.",
    "with its brooding, opaque character, suggesting rather than revealing power at this stage.",
    "representing the epitome of the power and concentration that lies behind the reputation of the appellation.",
    "is the best of the collection, with its brooding, opaque character.",
    "offering a symphony of flavors.",
    "delivering a harmonious blend of complexity and richness.",
    "capturing the essence of the region's unique terroir.",
    "exuding sophistication and finesse.",
    "showcasing the depth and intensity of its origin.",
    "revealing layers of intricate flavors.",
    "bringing forward notes of elegance and power.",
    "exemplifying the balance and grace typical of the region.",
    "with a remarkable expression of local terroir.",
    "highlighting the unique characteristics of the vintage.",
    "displaying a masterful use of traditional techniques.",
    "is crafted to perfection.",
    "with a rich, complex profile.",
    "combining tradition and innovation.",
    "demonstrating exceptional winemaking skills."
]

phrases_end = [
    "This wine will age gracefully for decades.",
    "Enjoy it with a fine meal.",
    "Perfect for special occasions.",
    "A true gem for collectors.",
    "Its potential is only just beginning to show.",
    "An ideal companion for hearty dishes.",
    "Best enjoyed with close friends and family.",
    "A masterpiece in every sense.",
    "Ideal for a quiet evening of reflection.",
    "A sublime experience for the discerning palate.",
    "Pair it with your favorite gourmet dishes.",
    "Sure to impress even the most discerning connoisseurs.",
    "It will only get better with time.",
    "A delightful addition to any celebration.",
    "Its richness is matched by its finesse.",
    "This wine promises a memorable tasting experience.",
    "It's the pinnacle of the winemaker's art.",
    "An exceptional choice for your wine cellar.",
    "A versatile wine, perfect for various pairings.",
    "Enjoy its depth and complexity as it matures."
]


def generate_description():
    intro = random.choice(phrases_intro)
    middle = random.choice(phrases_middle)
    end = random.choice(phrases_end)
    return f"{intro} {middle} {end}"


def generate_wine(country, data):
    return {
        "title": random.choice(data["titles"]),
        "province": random.choice(data["provinces"]),
        "description": generate_description(),
        "points": random.randint(80, 100),
        "country": country,
        "price": round(random.uniform(10.0, 300.0), 2),
        "winery": random.choice(data["wineries"]),
    }


wines = []
for _ in range(1000):
    country = random.choice(list(wines_data.keys()))
    wine_data = wines_data[country]
    wines.append(generate_wine(country, wine_data))

with open("wines.json", "w") as f:
    json.dump(wines, f, indent=4)
