"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../../../hooks/useAuth";
import type { IntervenantFormData } from "../../../types/user";

import { StepProgressBar } from "@/components/form/StepProgressBar";
import { StepControls } from "@/components/form/StepControls";


const steps = [
    "Informations personnelles",
    "Disponibilités",
    "Conditions de travail",
    "Diplômes",
    "Expériences",
    "Compétences",
    "Documents",
];

export default function IntervenantRegister() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();

    const [currentStep, setCurrentStep] = useState(0);

    const [formData, setFormData] = useState<IntervenantFormData>({
        temp_info: {
            last_name: "",
            first_name: "",
            address: "",
            phone: "",
            birth_date: "",
            has_driver_license: false,
            transport_mode: "",
        },
        availabilities: [{ availability_periods: "", time_slots: "", geographic_zones: "", max_travel_time: "" }],
        conditions: [{ hourly_rate: "", contract_types: "", auto_entrepreneur: false }],
        diplomas: [{ main_diploma: "", other_certifications: "", graduation_year: "", school: "" }],
        experiences: [{ total_experience: "", previous_positions: "", structure_types: "", tasks: "" }],
        skills: [{ languages: "", pedagogies: "", special_skills: "", special_needs_handling: "" }],
        documents: [{ motivation_letter: "", professional_references: "", required_documents: "", criminal_record: "", diplomas: "" }],
    },);

    // Charge les données du formulaire si existantes dans localStorage
    useEffect(() => {
        if (typeof window !== "undefined") {
            const storedFormData = localStorage.getItem("formData");
            if (storedFormData) {
                setFormData(JSON.parse(storedFormData));
            }
        }
    }, []);

    //Sauvegarde des données à chaque changement
    useEffect(() => {
        if (typeof window !== "undefined") {
            localStorage.setItem("formData", JSON.stringify(formData));
        }
    }, [formData]);

    const handleNext = () => {
        if (currentStep < steps.length - 1) {
            setCurrentStep(currentStep + 1);
        }
    };

    const handleBack = () => {
        if (currentStep > 0) {
            setCurrentStep(currentStep - 1);
        }
    };

    const handleChange = (section: keyof IntervenantFormData, field: any, index?: number) => {
        setFormData((prev) => {
            const updated = { ...prev };
            if (Array.isArray(updated[section]) && typeof index === "number") {
                updated[section][index] = { ...updated[section][index], ...field };
            } else {
                updated[section] = { ...updated[section], ...field };
            }
            return updated;
        });
    };

    //Envoi du formulaire au back dès grâce au clic du bouton "Valider" (bouton type = submit)
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!token || !isAuthenticated) {
            alert("Token inexistant, connexion non authentifiée, redirection vers login");
            return;
        }
        console.log("DIPLOMAS AVANT MAP :", formData.diplomas);

        //Data formattée pour correspondre exactement à la struct attendue dans le back-end
        const preparedData = {
            temp_info: {
                last_name: formData.temp_info.last_name,
                first_name: formData.temp_info.first_name,
                address: formData.temp_info.address,
                phone: formData.temp_info.phone,
                birth_date: formData.temp_info.birth_date || null,
                has_driver_license: formData.temp_info.has_driver_license,
                transport_mode: formData.temp_info.transport_mode,
            },
            //Le back attend des []
            availabilities: Object.values(formData.availabilities),
            conditions: Object.values(formData.conditions),
            diplomas: Object.values(formData.diplomas).map((d) => ({
                ...d,
                graduation_year: parseInt(formData.diplomas[0].graduation_year, 10),
            })),
            experiences: Object.values(formData.experiences),
            skills: Object.values(formData.skills),
            documents: Object.values(formData.documents),
        };


        console.log("Prepared Data ready to be sent to server : \n", preparedData);
        try {

            const response = await fetch("http://127.0.0.1:8080/register/temp", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${token}`,
                },
                body: JSON.stringify(preparedData),
            });
            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(errorText);
            }
            // setTimeout(()=> router.push("/profile"), 2000);
            localStorage.removeItem("formData");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };

    return (
        <form>
            <div className="max-w-2xl mx-auto p-4">
                {currentStep === 6 && (
                    <p className="text-5x1">L'upload de docs ne marche pas pour le moment</p>
                )}

                <StepProgressBar steps={steps} currentStep={currentStep} />

                {/* Ici on affiche un composant différent selon currentStep */}
                {/* Informations personnelles */}
                {currentStep === 0 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.temp_info.last_name}
                                onChange={(e) => handleChange("temp_info", { last_name: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Nom"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.temp_info.first_name}
                                onChange={(e) => handleChange("temp_info", { first_name: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Prénom"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="date"
                                max="2007-04-18"
                                value={formData.temp_info.birth_date}
                                onChange={(e) => handleChange("temp_info", { birth_date: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Date de naissance"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.temp_info.address}
                                onChange={(e) => handleChange("temp_info", { address: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Adresse complète"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="tel"
                                value={formData.temp_info.phone}
                                onChange={(e) => handleChange("temp_info", { phone: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Numéro de téléphone"
                            />
                        </label>
                        <label className="block">
                            <select
                                onChange={(e) => handleChange("temp_info", { has_driver_license: e.target.value === "true" ? true : false })}
                                className="w-full border p-2 rounded"
                            >
                                <option value="">Permis de conduire ?</option>
                                <option value="true">Oui</option>
                                <option value="false">Non</option>
                            </select>
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.temp_info.transport_mode}
                                onChange={(e) => handleChange("temp_info", { transport_mode: e.target.value })}
                                className="w-full border p-2 rounded"
                                placeholder="Moyens de transport"
                            />
                        </label>
                    </div>
                )}
                {/* Disponibilités */}
                {currentStep === 1 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.availabilities[0].availability_periods}
                                onChange={(e) =>
                                    handleChange("availabilities", [{ ...formData.availabilities[0], availability_periods: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Périodes disponibles (ex: vacances scolaires, toute l'année...)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.availabilities[0].time_slots}
                                onChange={(e) =>
                                    handleChange("availabilities", [{ ...formData.availabilities[0], time_slots: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Plages horaires de travail (ex: matin, après-midi, soir)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.availabilities[0].geographic_zones}
                                onChange={(e) =>
                                    handleChange("availabilities", [{ ...formData.availabilities[0], geographic_zones: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Lieux de travail privilégiés (ex: villes, quartiers...)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.availabilities[0].max_travel_time}
                                onChange={(e) =>
                                    handleChange("availabilities", [{ ...formData.availabilities[0], max_travel_time: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Temps de trajet maximal accepté (en minutes)"
                            />
                        </label>
                    </div>
                )}
                {/* Conditions de travail */}
                {currentStep === 2 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.conditions[0].hourly_rate}
                                onChange={(e) =>
                                    handleChange("conditions", [{ ...formData.conditions[0], hourly_rate: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Taux horaire souhaité (€)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.conditions[0].contract_types}
                                onChange={(e) =>
                                    handleChange("conditions", [{ ...formData.conditions[0], contract_types: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Types de contrat acceptés (ex: CDI, CDD, mission ponctuelle...)"
                            />
                        </label>
                        <label className="block">
                            <select
                                onChange={(e) => handleChange("conditions", [{ ...formData.conditions[0], auto_entrepreneur: e.target.value === "true"}])}
                                className="w-full border p-2 rounded"
                            >
                                <option value="">Préférez-vous être facturé en tant qu'auto-entrepreneur ?</option>
                                <option value="true">Oui</option>
                                <option value="false">Non</option>
                            </select>
                        </label>
                    </div>
                )}
                {/* Diplômes */}
                {currentStep === 3 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.diplomas[0].main_diploma}
                                onChange={(e) =>
                                    handleChange("diplomas", [{ ...formData.diplomas[0], main_diploma: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Intitulé de votre dernier diplôme"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.diplomas[0].other_certifications}
                                onChange={(e) =>
                                    handleChange("diplomas", [{ ...formData.diplomas[0], other_certifications: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Autres formations ou certifications"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.diplomas[0].graduation_year}
                                onChange={(e) =>
                                    handleChange("diplomas", [{ ...formData.diplomas[0], graduation_year: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Année(s) d'obtention"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.diplomas[0].school}
                                onChange={(e) =>
                                    handleChange("diplomas", [{ ...formData.diplomas[0], school: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Établissement(s) de formation"
                            />
                        </label>
                    </div>
                )}

                {/* Expérience(s) professionnelle(s) */}
                {currentStep === 4 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.experiences[0].total_experience}
                                onChange={(e) =>
                                    handleChange("experiences", [{ ...formData.experiences[0], total_experience: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Nombre total d'années d'expérience dans la petite enfance"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.experiences[0].previous_positions}
                                onChange={(e) =>
                                    handleChange("experiences", [{ ...formData.experiences[0], previous_positions: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Poste(s) précédemment occupé(s)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.experiences[0].structure_types}
                                onChange={(e) =>
                                    handleChange("experiences", [{ ...formData.experiences[0], structure_types: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Types de structures (ex: crèche, domicile...)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.experiences[0].tasks}
                                onChange={(e) =>
                                    handleChange("experiences", [{ ...formData.experiences[0], tasks: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Tâches réalisées"
                            />
                        </label>
                    </div>
                )}

                {/* Compétences */}
                {currentStep === 5 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.skills[0].languages}
                                onChange={(e) =>
                                    handleChange("skills", [{ ...formData.skills[0], languages: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Langues parlées"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.skills[0].pedagogies}
                                onChange={(e) =>
                                    handleChange("skills", [{ ...formData.skills[0], pedagogies: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Pédagogies utilisées (ex: Montessori, bienveillance...)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.skills[0].special_skills}
                                onChange={(e) =>
                                    handleChange("skills", [{ ...formData.skills[0], special_skills: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Savoirs-faire spécialisés (ex: éveil musical, ateliers créatifs...)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.skills[0].special_needs_handling}
                                onChange={(e) =>
                                    handleChange("skills", [{ ...formData.skills[0], special_needs_handling: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Gestion des besoins spécifiques (ex: allergies, handicap...)"
                            />
                        </label>
                    </div>
                )}

                {/* Documents à fournir */}
                {currentStep === 6 && (
                    <div className="space-y-4">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.documents[0].motivation_letter}
                                onChange={(e) =>
                                    handleChange("documents", [{ ...formData.documents[0], motivation_letter: e.target.value }])
                                }
                                className="w-full border p-2 rounded"
                                placeholder="Dites-nous pourquoi vous aimez ou voulez travailler dans la petite enfance."
                            />
                        </label>
                        <label className="block">
                            <span className="text-sm font-medium">Références professionnelles</span>
                            <div className="flex items-center space-x-2">
                                <input
                                    type="file"
                                    id="professional_references"
                                    onChange={(e) =>
                                        handleChange("documents", [{ ...formData.documents[0], professional_references: e.target.files?.[0] || "" }])
                                    }
                                    className="hidden"
                                />
                                <label
                                    htmlFor="professional_refs"
                                    className="cursor-pointer bg-blue-500 hover:bg-blue-600 text-white text-sm px-3 py-1 rounded"
                                >
                                    Choisir un fichier
                                </label>
                                <span className="text-gray-500 text-sm">
                                    {formData.documents[0].professional_references
                                        ? typeof formData.documents[0].professional_references === "string"
                                            ? formData.documents[0].professional_references[0]
                                            : formData.documents[0].professional_references
                                        : "Aucun fichier sélectionné"}
                                </span>
                            </div>
                        </label>
                        <label className="block">
                            <span className="text-sm font-medium">Pièces obligatoires d'identité (CNI, permis...)</span>
                            <div className="flex items-center space-x-2">
                                <input
                                    type="file"
                                    onChange={(e) =>
                                        handleChange("documents", [{ ...formData.documents[0], required_documentss: e.target.files?.[0] || "" }])
                                    }
                                    className="w-full border p-2 rounded"
                                />
                            </div>
                        </label>
                        <label className="block">
                            <span className="text-sm font-medium">Casier judiciaire (Bulletin n°3)</span>
                            <input
                                type="file"
                                onChange={(e) =>
                                    handleChange("documents", [{ ...formData.documents[0], criminal_record: e.target.files?.[0] || "" }])
                                }
                                className="w-full border p-2 rounded"
                            />
                        </label>
                        <label className="block">
                            <span className="text-sm font-medium">Diplômes</span>
                            <input
                                type="file"
                                onChange={(e) =>
                                    handleChange("documents", [{ ...formData.documents[0], diplomas: e.target.files?.[0] || "" }])
                                }
                                className="w-full border p-2 rounded"
                            />
                        </label>
                    </div>
                )}

                {/*Composant permettant d'interagir avec le forumulaire (retour, suivant, envoyer) */}
                <StepControls
                    currentStep={currentStep}
                    totalSteps={steps.length}
                    onNext={handleNext}
                    onBack={handleBack}
                    onSubmit={handleSubmit}
                />
            </div>
        </form>

    );
}
