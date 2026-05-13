#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>

#include "stdbr.h"
#include "cJSON.h"

static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;

#define ASSERT_STR_EQ(got, expected, label) do { \
    const char *_g = (got); \
    const char *_e = (expected); \
    tests_run++; \
    if (_g == NULL && _e == NULL) { tests_passed++; } \
    else if (_g == NULL || _e == NULL || strcmp(_g, _e) != 0) { \
        fprintf(stderr, "FAIL [%s]: got \"%s\", expected \"%s\"\n", label, _g ? _g : "(null)", _e ? _e : "(null)"); \
        tests_failed++; \
    } else { tests_passed++; } \
} while(0)

#define ASSERT_INT_EQ(got, expected, label) do { \
    long long _g = (long long)(got); \
    long long _e = (long long)(expected); \
    tests_run++; \
    if (_g != _e) { \
        fprintf(stderr, "FAIL [%s]: got %lld, expected %lld\n", label, _g, _e); \
        tests_failed++; \
    } else { tests_passed++; } \
} while(0)

#define ASSERT_BOOL_EQ(got, expected, label) do { \
    bool _g = (got); \
    bool _e = (expected); \
    tests_run++; \
    if (_g != _e) { \
        fprintf(stderr, "FAIL [%s]: got %s, expected %s\n", label, _g ? "true" : "false", _e ? "true" : "false"); \
        tests_failed++; \
    } else { tests_passed++; } \
} while(0)

#define ASSERT_NOT_NULL(ptr, label) do { \
    tests_run++; \
    if ((ptr) == NULL) { \
        fprintf(stderr, "FAIL [%s]: expected non-null\n", label); \
        tests_failed++; \
    } else { tests_passed++; } \
} while(0)

#define ASSERT_NULL(ptr, label) do { \
    tests_run++; \
    if ((ptr) != NULL) { \
        fprintf(stderr, "FAIL [%s]: expected null\n", label); \
        tests_failed++; \
    } else { tests_passed++; } \
} while(0)

static void assert_str_eq_free(char *got, const char *expected, const char *label) {
    ASSERT_STR_EQ(got, expected, label);
    if (got) stdbr_free(got);
}

static StdbrCpfError cpf_error_from_str(const char *s) {
    if (!s) return 0;
    if (strstr(s, "exactly 11 digits")) return 1;
    if (strstr(s, "invalid characters")) return 2;
    if (strstr(s, "format must be")) return 3;
    if (strstr(s, "all equal digits")) return 4;
    if (strstr(s, "check digits are invalid")) return 5;
    fprintf(stderr, "WARNING: unknown CPF error string: %s\n", s);
    return 255;
}

static StdbrCnpjError cnpj_error_from_str(const char *s) {
    if (!s) return 0;
    if (strstr(s, "exactly 14 characters")) return 1;
    if (strstr(s, "invalid characters")) return 2;
    if (strstr(s, "format must be")) return 3;
    if (strstr(s, "all equal characters")) return 4;
    if (strstr(s, "check digits are invalid")) return 5;
    fprintf(stderr, "WARNING: unknown CNPJ error string: %s\n", s);
    return 255;
}

static StdbrCepError cep_error_from_str(const char *s) {
    if (!s) return 0;
    if (strstr(s, "exactly 8 digits")) return 1;
    if (strstr(s, "invalid characters")) return 2;
    if (strstr(s, "format must be")) return 3;
    fprintf(stderr, "WARNING: unknown CEP error string: %s\n", s);
    return 255;
}

static void test_cpf(cJSON *cpf_json) {
    printf("  CPF...\n");

    cJSON *parse = cJSON_GetObjectItem(cpf_json, "parse");
    cJSON *item;
    cJSON_ArrayForEach(item, parse) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *digits_only = cJSON_GetObjectItem(item, "digits_only")->valuestring;
        const char *formatted = cJSON_GetObjectItem(item, "formatted")->valuestring;
        const char *masked = cJSON_GetObjectItem(item, "masked")->valuestring;
        int fiscal_region = cJSON_GetObjectItem(item, "fiscal_region")->valueint;
        cJSON *cd = cJSON_GetObjectItem(item, "check_digits");
        int cd0 = cJSON_GetArrayItem(cd, 0)->valueint;
        int cd1 = cJSON_GetArrayItem(cd, 1)->valueint;

        StdbrCpfError err;
        StdbrCpf *cpf = stdbr_cpf_parse(input, &err);
        ASSERT_INT_EQ(err, 0, "cpf_parse err");
        ASSERT_NOT_NULL(cpf, "cpf_parse result");
        if (!cpf) continue;

        assert_str_eq_free(stdbr_cpf_as_str(cpf), digits_only, "cpf as_str");
        assert_str_eq_free(stdbr_cpf_formatted(cpf), formatted, "cpf formatted");
        assert_str_eq_free(stdbr_cpf_masked(cpf), masked, "cpf masked");
        ASSERT_INT_EQ(stdbr_cpf_fiscal_region(cpf), fiscal_region, "cpf fiscal_region");

        uint8_t d1, d2;
        stdbr_cpf_check_digits(cpf, &d1, &d2);
        ASSERT_INT_EQ(d1, cd0, "cpf check_digit[0]");
        ASSERT_INT_EQ(d2, cd1, "cpf check_digit[1]");

        stdbr_cpf_destroy(cpf);
    }

    cJSON *is_valid = cJSON_GetObjectItem(cpf_json, "is_valid");
    cJSON_ArrayForEach(item, is_valid) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool expected = cJSON_IsTrue(cJSON_GetObjectItem(item, "expected"));
        bool got = stdbr_cpf_is_valid(input);
        ASSERT_BOOL_EQ(got, expected, "cpf is_valid");
    }

    cJSON *strict = cJSON_GetObjectItem(cpf_json, "is_valid_strict");
    cJSON_ArrayForEach(item, strict) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool valid = cJSON_IsTrue(cJSON_GetObjectItem(item, "valid"));
        StdbrCpfError err = stdbr_cpf_is_valid_strict(input);
        if (valid) {
            ASSERT_INT_EQ(err, 0, "cpf strict valid");
        } else {
            const char *error_str = cJSON_GetObjectItem(item, "error")->valuestring;
            StdbrCpfError expected_err = cpf_error_from_str(error_str);
            ASSERT_INT_EQ(err, expected_err, "cpf strict error");
        }
    }

    cJSON *fmt = cJSON_GetObjectItem(cpf_json, "format");
    cJSON_ArrayForEach(item, fmt) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        char *got = stdbr_cpf_format(input);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_NULL(got, "cpf format null");
        } else {
            assert_str_eq_free(got, exp_json->valuestring, "cpf format");
            got = NULL;
        }
        if (got) stdbr_free(got);
    }

    cJSON *rs = cJSON_GetObjectItem(cpf_json, "remove_symbols");
    cJSON_ArrayForEach(item, rs) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *expected = cJSON_GetObjectItem(item, "expected")->valuestring;
        assert_str_eq_free(stdbr_cpf_remove_symbols(input), expected, "cpf remove_symbols");
    }

    cJSON *ccd = cJSON_GetObjectItem(cpf_json, "compute_check_digits");
    cJSON_ArrayForEach(item, ccd) {
        const char *base = cJSON_GetObjectItem(item, "base")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        uint8_t d1 = 0, d2 = 0;
        bool ok = stdbr_cpf_compute_check_digits(base, &d1, &d2);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_BOOL_EQ(ok, false, "cpf compute_check_digits null");
        } else {
            ASSERT_BOOL_EQ(ok, true, "cpf compute_check_digits ok");
            ASSERT_INT_EQ(d1, cJSON_GetArrayItem(exp_json, 0)->valueint, "cpf cd[0]");
            ASSERT_INT_EQ(d2, cJSON_GetArrayItem(exp_json, 1)->valueint, "cpf cd[1]");
        }
    }

    char *generated = stdbr_cpf_generate();
    ASSERT_NOT_NULL(generated, "cpf generate");
    if (generated) {
        ASSERT_BOOL_EQ(stdbr_cpf_is_valid(generated), true, "cpf generate valid");
        StdbrCpfError err;
        StdbrCpf *cpf = stdbr_cpf_parse(generated, &err);
        ASSERT_NOT_NULL(cpf, "cpf generate parse");
        if (cpf) {
            char *s = stdbr_cpf_as_str(cpf);
            ASSERT_STR_EQ(s, generated, "cpf generate roundtrip");
            stdbr_free(s);
            stdbr_cpf_destroy(cpf);
        }
        stdbr_free(generated);
    }
}

static void test_cnpj(cJSON *cnpj_json) {
    printf("  CNPJ...\n");

    cJSON *parse = cJSON_GetObjectItem(cnpj_json, "parse");
    cJSON *item;
    cJSON_ArrayForEach(item, parse) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *digits_only = cJSON_GetObjectItem(item, "digits_only")->valuestring;
        const char *formatted = cJSON_GetObjectItem(item, "formatted")->valuestring;
        const char *masked = cJSON_GetObjectItem(item, "masked")->valuestring;
        int kind = cJSON_GetObjectItem(item, "kind")->valueint;
        int est_type = cJSON_GetObjectItem(item, "establishment_type")->valueint;
        cJSON *cd = cJSON_GetObjectItem(item, "check_digits");
        int cd0 = cJSON_GetArrayItem(cd, 0)->valueint;
        int cd1 = cJSON_GetArrayItem(cd, 1)->valueint;

        StdbrCnpjError err;
        StdbrCnpj *cnpj = stdbr_cnpj_parse(input, &err);
        ASSERT_INT_EQ(err, 0, "cnpj_parse err");
        ASSERT_NOT_NULL(cnpj, "cnpj_parse result");
        if (!cnpj) continue;

        assert_str_eq_free(stdbr_cnpj_as_str(cnpj), digits_only, "cnpj as_str");
        assert_str_eq_free(stdbr_cnpj_formatted(cnpj), formatted, "cnpj formatted");
        assert_str_eq_free(stdbr_cnpj_masked(cnpj), masked, "cnpj masked");
        ASSERT_INT_EQ(stdbr_cnpj_kind(cnpj), kind, "cnpj kind");
        ASSERT_INT_EQ(stdbr_cnpj_establishment_type(cnpj), est_type, "cnpj establishment_type");

        uint8_t d1, d2;
        stdbr_cnpj_check_digits(cnpj, &d1, &d2);
        ASSERT_INT_EQ(d1, cd0, "cnpj check_digit[0]");
        ASSERT_INT_EQ(d2, cd1, "cnpj check_digit[1]");

        stdbr_cnpj_destroy(cnpj);
    }

    cJSON *is_valid = cJSON_GetObjectItem(cnpj_json, "is_valid");
    cJSON_ArrayForEach(item, is_valid) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool expected = cJSON_IsTrue(cJSON_GetObjectItem(item, "expected"));
        bool got = stdbr_cnpj_is_valid(input);
        ASSERT_BOOL_EQ(got, expected, "cnpj is_valid");
    }

    cJSON *strict = cJSON_GetObjectItem(cnpj_json, "is_valid_strict");
    cJSON_ArrayForEach(item, strict) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool valid = cJSON_IsTrue(cJSON_GetObjectItem(item, "valid"));
        StdbrCnpjError err = stdbr_cnpj_is_valid_strict(input);
        if (valid) {
            ASSERT_INT_EQ(err, 0, "cnpj strict valid");
        } else {
            const char *error_str = cJSON_GetObjectItem(item, "error")->valuestring;
            StdbrCnpjError expected_err = cnpj_error_from_str(error_str);
            ASSERT_INT_EQ(err, expected_err, "cnpj strict error");
        }
    }

    cJSON *fmt = cJSON_GetObjectItem(cnpj_json, "format");
    cJSON_ArrayForEach(item, fmt) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        char *got = stdbr_cnpj_format(input);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_NULL(got, "cnpj format null");
        } else {
            assert_str_eq_free(got, exp_json->valuestring, "cnpj format");
            got = NULL;
        }
        if (got) stdbr_free(got);
    }

    cJSON *rs = cJSON_GetObjectItem(cnpj_json, "remove_symbols");
    cJSON_ArrayForEach(item, rs) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *expected = cJSON_GetObjectItem(item, "expected")->valuestring;
        assert_str_eq_free(stdbr_cnpj_remove_symbols(input), expected, "cnpj remove_symbols");
    }

    cJSON *ccd = cJSON_GetObjectItem(cnpj_json, "compute_check_digits");
    cJSON_ArrayForEach(item, ccd) {
        const char *base = cJSON_GetObjectItem(item, "base")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        uint8_t d1 = 0, d2 = 0;
        bool ok = stdbr_cnpj_compute_check_digits(base, &d1, &d2);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_BOOL_EQ(ok, false, "cnpj compute_check_digits null");
        } else {
            ASSERT_BOOL_EQ(ok, true, "cnpj compute_check_digits ok");
            ASSERT_INT_EQ(d1, cJSON_GetArrayItem(exp_json, 0)->valueint, "cnpj cd[0]");
            ASSERT_INT_EQ(d2, cJSON_GetArrayItem(exp_json, 1)->valueint, "cnpj cd[1]");
        }
    }

    char *generated = stdbr_cnpj_generate(0);
    ASSERT_NOT_NULL(generated, "cnpj generate");
    if (generated) {
        ASSERT_BOOL_EQ(stdbr_cnpj_is_valid(generated), true, "cnpj generate valid");
        StdbrCnpjError err;
        StdbrCnpj *cnpj = stdbr_cnpj_parse(generated, &err);
        ASSERT_NOT_NULL(cnpj, "cnpj generate parse");
        if (cnpj) {
            char *s = stdbr_cnpj_as_str(cnpj);
            ASSERT_STR_EQ(s, generated, "cnpj generate roundtrip");
            stdbr_free(s);
            stdbr_cnpj_destroy(cnpj);
        }
        stdbr_free(generated);
    }
}

static void test_cep(cJSON *cep_json) {
    printf("  CEP...\n");

    cJSON *parse = cJSON_GetObjectItem(cep_json, "parse");
    cJSON *item;
    cJSON_ArrayForEach(item, parse) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *digits_only = cJSON_GetObjectItem(item, "digits_only")->valuestring;
        const char *formatted = cJSON_GetObjectItem(item, "formatted")->valuestring;
        const char *masked = cJSON_GetObjectItem(item, "masked")->valuestring;
        int postal_region = cJSON_GetObjectItem(item, "postal_region")->valueint;
        cJSON *state_json = cJSON_GetObjectItem(item, "state");

        StdbrCepError err;
        StdbrCep *cep = stdbr_cep_parse(input, &err);
        ASSERT_INT_EQ(err, 0, "cep_parse err");
        ASSERT_NOT_NULL(cep, "cep_parse result");
        if (!cep) continue;

        assert_str_eq_free(stdbr_cep_as_str(cep), digits_only, "cep as_str");
        assert_str_eq_free(stdbr_cep_formatted(cep), formatted, "cep formatted");
        assert_str_eq_free(stdbr_cep_masked(cep), masked, "cep masked");
        ASSERT_INT_EQ(stdbr_cep_postal_region(cep), postal_region, "cep postal_region");

        StdbrState state_out;
        bool has_state = stdbr_cep_state(cep, &state_out);
        if (cJSON_IsNull(state_json)) {
            ASSERT_BOOL_EQ(has_state, false, "cep state null");
        } else {
            ASSERT_BOOL_EQ(has_state, true, "cep has state");
            if (has_state) {
                char *abbr = stdbr_state_abbreviation(state_out);
                ASSERT_STR_EQ(abbr, state_json->valuestring, "cep state abbr");
                stdbr_free(abbr);
            }
        }

        stdbr_cep_destroy(cep);
    }

    cJSON *is_valid = cJSON_GetObjectItem(cep_json, "is_valid");
    cJSON_ArrayForEach(item, is_valid) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool expected = cJSON_IsTrue(cJSON_GetObjectItem(item, "expected"));
        bool got = stdbr_cep_is_valid(input);
        ASSERT_BOOL_EQ(got, expected, "cep is_valid");
    }

    cJSON *strict = cJSON_GetObjectItem(cep_json, "is_valid_strict");
    cJSON_ArrayForEach(item, strict) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool valid = cJSON_IsTrue(cJSON_GetObjectItem(item, "valid"));
        StdbrCepError err = stdbr_cep_is_valid_strict(input);
        if (valid) {
            ASSERT_INT_EQ(err, 0, "cep strict valid");
        } else {
            const char *error_str = cJSON_GetObjectItem(item, "error")->valuestring;
            StdbrCepError expected_err = cep_error_from_str(error_str);
            ASSERT_INT_EQ(err, expected_err, "cep strict error");
        }
    }

    cJSON *fmt = cJSON_GetObjectItem(cep_json, "format");
    cJSON_ArrayForEach(item, fmt) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        char *got = stdbr_cep_format(input);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_NULL(got, "cep format null");
        } else {
            assert_str_eq_free(got, exp_json->valuestring, "cep format");
            got = NULL;
        }
        if (got) stdbr_free(got);
    }

    cJSON *rs = cJSON_GetObjectItem(cep_json, "remove_symbols");
    cJSON_ArrayForEach(item, rs) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *expected = cJSON_GetObjectItem(item, "expected")->valuestring;
        assert_str_eq_free(stdbr_cep_remove_symbols(input), expected, "cep remove_symbols");
    }

    char *generated = stdbr_cep_generate();
    ASSERT_NOT_NULL(generated, "cep generate");
    if (generated) {
        ASSERT_BOOL_EQ(stdbr_cep_is_valid(generated), true, "cep generate valid");
        StdbrCepError err;
        StdbrCep *cep = stdbr_cep_parse(generated, &err);
        ASSERT_NOT_NULL(cep, "cep generate parse");
        if (cep) {
            char *s = stdbr_cep_as_str(cep);
            ASSERT_STR_EQ(s, generated, "cep generate roundtrip");
            stdbr_free(s);
            stdbr_cep_destroy(cep);
        }
        stdbr_free(generated);
    }
}

static StdbrRgError rg_error_from_str(const char *s) {
    if (!s) return 0;
    if (strstr(s, "outside the accepted range")) return 1;
    if (strstr(s, "invalid characters")) return 2;
    if (strstr(s, "does not match the canonical mask")) return 3;
    if (strstr(s, "check digit is invalid")) return 4;
    if (strstr(s, "generation is not supported")) return 5;
    fprintf(stderr, "WARNING: unknown RG error string: %s\n", s);
    return 255;
}

static bool uf_from_json(cJSON *item, const char *key, StdbrState *out) {
    const char *abbr = cJSON_GetObjectItem(item, key)->valuestring;
    return stdbr_state_from_abbreviation(abbr, out);
}

static void test_rg(cJSON *rg_json) {
    printf("  RG...\n");

    cJSON *parse = cJSON_GetObjectItem(rg_json, "parse");
    cJSON *item;
    cJSON_ArrayForEach(item, parse) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *digits_only = cJSON_GetObjectItem(item, "digits_only")->valuestring;
        const char *formatted = cJSON_GetObjectItem(item, "formatted")->valuestring;
        const char *uf_out = cJSON_GetObjectItem(item, "uf_out")->valuestring;
        cJSON *cd_json = cJSON_GetObjectItem(item, "check_digit");

        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) {
            ASSERT_BOOL_EQ(false, true, "rg parse uf lookup");
            continue;
        }

        StdbrRgError err;
        StdbrRg *rg = stdbr_rg_parse(input, uf, &err);
        ASSERT_INT_EQ(err, 0, "rg_parse err");
        ASSERT_NOT_NULL(rg, "rg_parse result");
        if (!rg) continue;

        assert_str_eq_free(stdbr_rg_as_str(rg), digits_only, "rg as_str");
        assert_str_eq_free(stdbr_rg_formatted(rg), formatted, "rg formatted");

        StdbrState got_uf = stdbr_rg_uf(rg);
        char *got_uf_abbr = stdbr_state_abbreviation(got_uf);
        ASSERT_STR_EQ(got_uf_abbr, uf_out, "rg uf abbr");
        stdbr_free(got_uf_abbr);

        uint8_t cd = 0;
        bool has_cd = stdbr_rg_check_digit(rg, &cd);
        if (cJSON_IsNull(cd_json)) {
            ASSERT_BOOL_EQ(has_cd, false, "rg check_digit absent");
        } else {
            ASSERT_BOOL_EQ(has_cd, true, "rg check_digit present");
            ASSERT_INT_EQ(cd, cd_json->valueint, "rg check_digit value");
        }

        stdbr_rg_destroy(rg);
    }

    cJSON *is_valid = cJSON_GetObjectItem(rg_json, "is_valid");
    cJSON_ArrayForEach(item, is_valid) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool expected = cJSON_IsTrue(cJSON_GetObjectItem(item, "expected"));
        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) continue;
        ASSERT_BOOL_EQ(stdbr_rg_is_valid(input, uf), expected, "rg is_valid");
    }

    cJSON *strict = cJSON_GetObjectItem(rg_json, "is_valid_strict");
    cJSON_ArrayForEach(item, strict) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        bool valid = cJSON_IsTrue(cJSON_GetObjectItem(item, "valid"));
        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) continue;
        StdbrRgError err = stdbr_rg_is_valid_strict(input, uf);
        if (valid) {
            ASSERT_INT_EQ(err, 0, "rg strict valid");
        } else {
            const char *error_str = cJSON_GetObjectItem(item, "error")->valuestring;
            StdbrRgError expected_err = rg_error_from_str(error_str);
            ASSERT_INT_EQ(err, expected_err, "rg strict error");
        }
    }

    cJSON *fmt = cJSON_GetObjectItem(rg_json, "format");
    cJSON_ArrayForEach(item, fmt) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) continue;
        char *got = stdbr_rg_format(input, uf);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_NULL(got, "rg format null");
        } else {
            assert_str_eq_free(got, exp_json->valuestring, "rg format");
            got = NULL;
        }
        if (got) stdbr_free(got);
    }

    cJSON *rs = cJSON_GetObjectItem(rg_json, "remove_symbols");
    cJSON_ArrayForEach(item, rs) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        const char *expected = cJSON_GetObjectItem(item, "expected")->valuestring;
        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) continue;
        assert_str_eq_free(stdbr_rg_remove_symbols(input, uf), expected, "rg remove_symbols");
    }

    cJSON *ccd = cJSON_GetObjectItem(rg_json, "compute_check_digit");
    cJSON_ArrayForEach(item, ccd) {
        const char *base = cJSON_GetObjectItem(item, "base")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");
        StdbrState uf;
        if (!uf_from_json(item, "uf", &uf)) continue;
        uint8_t out = 0;
        bool ok = stdbr_rg_compute_check_digit(base, uf, &out);
        if (cJSON_IsNull(exp_json)) {
            ASSERT_BOOL_EQ(ok, false, "rg compute_check_digit null");
        } else {
            ASSERT_BOOL_EQ(ok, true, "rg compute_check_digit ok");
            ASSERT_INT_EQ(out, exp_json->valueint, "rg compute_check_digit value");
        }
    }

    StdbrState gen_uf;
    cJSON *gen = cJSON_GetObjectItem(rg_json, "generate");
    const char *gen_uf_abbr = cJSON_GetObjectItem(gen, "uf")->valuestring;
    if (stdbr_state_from_abbreviation(gen_uf_abbr, &gen_uf)) {
        StdbrRgError gen_err;
        char *generated = stdbr_rg_generate(gen_uf, &gen_err);
        ASSERT_NOT_NULL(generated, "rg generate");
        if (generated) {
            ASSERT_BOOL_EQ(stdbr_rg_is_valid(generated, gen_uf), true, "rg generate valid");
            StdbrRgError err;
            StdbrRg *rg = stdbr_rg_parse(generated, gen_uf, &err);
            ASSERT_NOT_NULL(rg, "rg generate parse");
            if (rg) {
                char *s = stdbr_rg_as_str(rg);
                ASSERT_STR_EQ(s, generated, "rg generate roundtrip");
                stdbr_free(s);
                stdbr_rg_destroy(rg);
            }
            stdbr_free(generated);
        }
    }

    cJSON *gen_unsup = cJSON_GetObjectItem(rg_json, "generate_unsupported");
    cJSON_ArrayForEach(item, gen_unsup) {
        const char *uf_abbr = cJSON_GetObjectItem(item, "uf")->valuestring;
        const char *err_str = cJSON_GetObjectItem(item, "error")->valuestring;
        StdbrState uf;
        if (!stdbr_state_from_abbreviation(uf_abbr, &uf)) continue;
        StdbrRgError err;
        StdbrRg *rg = stdbr_rg_create_for_uf(uf, &err);
        ASSERT_NULL(rg, "rg create_for_uf unsupported null");
        if (rg) stdbr_rg_destroy(rg);
        StdbrRgError expected_err = rg_error_from_str(err_str);
        ASSERT_INT_EQ(err, expected_err, "rg create_for_uf err");
    }
}

static void test_uf(cJSON *uf_json) {
    printf("  UF...\n");

    cJSON *states_json = cJSON_GetObjectItem(uf_json, "states");
    int expected_count = cJSON_GetArraySize(states_json);

    StdbrState buf[27];
    uint32_t count = stdbr_all_states(buf);
    ASSERT_INT_EQ(count, expected_count, "uf states count");

    for (int i = 0; i < (int)count && i < expected_count; i++) {
        cJSON *s = cJSON_GetArrayItem(states_json, i);
        const char *exp_abbr = cJSON_GetObjectItem(s, "abbreviation")->valuestring;
        const char *exp_name = cJSON_GetObjectItem(s, "name")->valuestring;

        char *abbr = stdbr_state_abbreviation(buf[i]);
        char *name = stdbr_state_name(buf[i]);
        ASSERT_STR_EQ(abbr, exp_abbr, "uf abbreviation");
        ASSERT_STR_EQ(name, exp_name, "uf name");
        stdbr_free(abbr);
        stdbr_free(name);
    }

    cJSON *from_abbr = cJSON_GetObjectItem(uf_json, "from_abbreviation");
    cJSON *item;
    cJSON_ArrayForEach(item, from_abbr) {
        const char *input = cJSON_GetObjectItem(item, "input")->valuestring;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");

        StdbrState out;
        bool found = stdbr_state_from_abbreviation(input, &out);

        if (cJSON_IsNull(exp_json)) {
            ASSERT_BOOL_EQ(found, false, "uf from_abbr null");
        } else {
            ASSERT_BOOL_EQ(found, true, "uf from_abbr found");
            if (found) {
                char *abbr = stdbr_state_abbreviation(out);
                ASSERT_STR_EQ(abbr, exp_json->valuestring, "uf from_abbr value");
                stdbr_free(abbr);
            }
        }
    }
}

static void test_municipio(cJSON *mun_json) {
    printf("  Municipio...\n");

    int expected_count = cJSON_GetObjectItem(mun_json, "count")->valueint;
    ASSERT_INT_EQ(stdbr_municipio_count(), expected_count, "municipio count");

    cJSON *from_code = cJSON_GetObjectItem(mun_json, "from_ibge_code");
    cJSON *item;
    cJSON_ArrayForEach(item, from_code) {
        uint32_t code = (uint32_t)cJSON_GetObjectItem(item, "code")->valueint;
        cJSON *exp_json = cJSON_GetObjectItem(item, "expected");

        StdbrMunicipio *m = stdbr_municipio_from_ibge_code(code);

        if (exp_json && cJSON_IsNull(exp_json)) {
            ASSERT_NULL(m, "municipio from_code null");
        } else {
            ASSERT_NOT_NULL(m, "municipio from_code found");
            if (m) {
                const char *exp_name = cJSON_GetObjectItem(item, "name")->valuestring;
                const char *exp_state = cJSON_GetObjectItem(item, "state")->valuestring;
                bool exp_capital = cJSON_IsTrue(cJSON_GetObjectItem(item, "is_capital"));

                char *name = stdbr_municipio_name(m);
                ASSERT_STR_EQ(name, exp_name, "municipio name");
                stdbr_free(name);

                StdbrState state = stdbr_municipio_state(m);
                char *abbr = stdbr_state_abbreviation(state);
                ASSERT_STR_EQ(abbr, exp_state, "municipio state");
                stdbr_free(abbr);

                ASSERT_BOOL_EQ(stdbr_municipio_is_capital(m), exp_capital, "municipio is_capital");
                stdbr_municipio_destroy(m);
            }
        }
    }

    cJSON *capitals = cJSON_GetObjectItem(mun_json, "capital_of");
    cJSON_ArrayForEach(item, capitals) {
        const char *state_str = cJSON_GetObjectItem(item, "state")->valuestring;
        const char *exp_name = cJSON_GetObjectItem(item, "name")->valuestring;
        uint32_t exp_code = (uint32_t)cJSON_GetObjectItem(item, "ibge_code")->valueint;

        StdbrState state;
        bool found = stdbr_state_from_abbreviation(state_str, &state);
        ASSERT_BOOL_EQ(found, true, "capital_of state lookup");
        if (!found) continue;

        StdbrMunicipio *cap = stdbr_municipio_capital_of(state);
        ASSERT_NOT_NULL(cap, "capital_of result");
        if (cap) {
            char *name = stdbr_municipio_name(cap);
            ASSERT_STR_EQ(name, exp_name, "capital_of name");
            stdbr_free(name);
            ASSERT_INT_EQ(stdbr_municipio_ibge_code(cap), exp_code, "capital_of ibge_code");
            stdbr_municipio_destroy(cap);
        }
    }

    cJSON *searches = cJSON_GetObjectItem(mun_json, "search_by_name");
    cJSON_ArrayForEach(item, searches) {
        const char *query = cJSON_GetObjectItem(item, "query")->valuestring;
        cJSON *results_json = cJSON_GetObjectItem(item, "results");
        int exp_len = cJSON_GetArraySize(results_json);

        StdbrMunicipioList *list = stdbr_municipio_search_by_name(query);
        ASSERT_NOT_NULL(list, "search_by_name list");
        if (!list) continue;

        uint32_t got_len = stdbr_municipio_list_count(list);
        ASSERT_INT_EQ(got_len, exp_len, "search_by_name count");

        int check_len = (int)got_len < exp_len ? (int)got_len : exp_len;
        for (int i = 0; i < check_len; i++) {
            cJSON *r = cJSON_GetArrayItem(results_json, i);
            uint32_t exp_ibge = (uint32_t)cJSON_GetObjectItem(r, "ibge_code")->valueint;
            const char *exp_rname = cJSON_GetObjectItem(r, "name")->valuestring;

            StdbrMunicipio *m = stdbr_municipio_list_get(list, (uint32_t)i);
            if (m) {
                ASSERT_INT_EQ(stdbr_municipio_ibge_code(m), exp_ibge, "search ibge_code");
                char *name = stdbr_municipio_name(m);
                ASSERT_STR_EQ(name, exp_rname, "search name");
                stdbr_free(name);
                stdbr_municipio_destroy(m);
            }
        }
        stdbr_municipio_list_destroy(list);
    }

    cJSON *by_state = cJSON_GetObjectItem(mun_json, "by_state_count");
    cJSON_ArrayForEach(item, by_state) {
        const char *state_str = cJSON_GetObjectItem(item, "state")->valuestring;
        int exp_cnt = cJSON_GetObjectItem(item, "count")->valueint;

        StdbrState state;
        bool found = stdbr_state_from_abbreviation(state_str, &state);
        ASSERT_BOOL_EQ(found, true, "by_state state lookup");
        if (!found) continue;

        StdbrMunicipioList *list = stdbr_municipio_by_state(state);
        ASSERT_NOT_NULL(list, "by_state list");
        if (list) {
            ASSERT_INT_EQ(stdbr_municipio_list_count(list), exp_cnt, "by_state count");
            stdbr_municipio_list_destroy(list);
        }
    }
}

int main(void) {
    const char *golden_path = getenv("GOLDEN_JSON");
    if (!golden_path) {
        fprintf(stderr, "ERROR: GOLDEN_JSON environment variable not set\n");
        return 1;
    }

    FILE *f = fopen(golden_path, "r");
    if (!f) {
        fprintf(stderr, "ERROR: cannot open %s\n", golden_path);
        return 1;
    }

    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buf = malloc((size_t)len + 1);
    if (!buf) {
        fclose(f);
        fprintf(stderr, "ERROR: malloc failed\n");
        return 1;
    }
    fread(buf, 1, (size_t)len, f);
    buf[len] = '\0';
    fclose(f);

    cJSON *golden = cJSON_Parse(buf);
    free(buf);
    if (!golden) {
        fprintf(stderr, "ERROR: failed to parse golden.json\n");
        return 1;
    }

    printf("Running FFI-C parity tests...\n");

    test_cpf(cJSON_GetObjectItem(golden, "cpf"));
    test_cnpj(cJSON_GetObjectItem(golden, "cnpj"));
    test_cep(cJSON_GetObjectItem(golden, "cep"));
    test_rg(cJSON_GetObjectItem(golden, "rg"));
    test_uf(cJSON_GetObjectItem(golden, "uf"));
    test_municipio(cJSON_GetObjectItem(golden, "municipio"));

    cJSON_Delete(golden);

    printf("\nResults: %d passed, %d failed, %d total\n", tests_passed, tests_failed, tests_run);

    return tests_failed > 0 ? 1 : 0;
}
