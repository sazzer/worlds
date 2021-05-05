import LanguageDetector from "i18next-browser-languagedetector";
import debug from "debug";
import defaultTranslations from "./messages.json";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

const LOG = debug("worlds:i18n");

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      dev: {
        translation: defaultTranslations,
      },
    },

    nsSeparator: false,

    debug: false,

    interpolation: {
      escapeValue: false,
      format: (value, format, lng) => {
        if (format === "intlDate") {
          return new Intl.DateTimeFormat(lng, {
            year: "numeric",
            month: "long",
            day: "numeric",
          }).format(value);
        } else if (format === "intlDateTime") {
          return new Intl.DateTimeFormat(lng, {
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "numeric",
            minute: "numeric",
            second: "numeric",
          }).format(value);
        }

        return value;
      },
    },

    parseMissingKeyHandler: (key) => {
      LOG("Request for unknown i18n key: %s", key);
      if (process.env.NODE_ENV === "test") {
        throw new Error(`Missing message key: ${key}`);
      } else {
        return `!!${key}!!`;
      }
    },
  });

export default i18n;
