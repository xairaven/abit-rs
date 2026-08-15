# Raw Data - Explanation

### Institutions

You can get them via link: https://registry.edbo.gov.ua/api/opendata/universities?rg=80&ut=1&exp=xlsx (of course, change parameter if needed)

**Parameters**:
- *rg* - Region. View [region chapter](#regions) for more information.
- *ut* - Institution categories. View [corresponding chapter](#institution-categories) for more information.
- *exp* - Output format. There are (at least): *.json*, *.xlsx*

### Institution Categories

There are (at least 6):
- Заклади вищої освіти - **Higher Education Institutions** (`HigherEducation`)
- Заклади фахової передвищої освіти - **Professional Pre-higher Education Institutions** (`ProfessionalCollege`)
- Заклади професійної професійно-технічної освіти -  **Vocational Education and Training Institutions** (`VocationalEducation`)
- Заклади загальної середньої освіти - **General Secondary Education Institutions** (`SecondaryEducation`)
- Наукові інститути (установи) - **Scientific Institutes** (`ScientificInstitutes`)
- Заклади післядипломної освіти - **Postgraduate Education Institutions** (`Postgrad`)

Also, in 2025 there were 2 additional categories. I don't know if they are exist now:
- Інший заклад освіти, що надає професійну (професійно-технічну освіту) - **Other Institution Providing Vocational Education and Training** (`OtherVET`)
- Невідомо - **Unknown** (`Unknown`)

Each category has its own numerical code. You can find current codes in [category.rs](../model/src/institution/category.rs).


### Regions

Each region has a numerical code. You can find current codes in [region.rs](../model/src/region.rs). 
Got them from [EDBO Registry](https://registry.edbo.gov.ua/vishcha-osvita) - there is interactive map.