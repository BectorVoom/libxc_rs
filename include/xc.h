/* libxc_rs — pure-Rust drop-in (source-level) replacement for libxc 7.0.0.
 *
 * SPDX-License-Identifier: MPL-2.0
 * Copyright (C) 2026 libxc_rs contributors.
 * Based on libxc 7.0.0 by M. A. L. Marques, S. Lehtola, et al.
 *
 * SOURCE-LEVEL DROP-IN: this header mirrors libxc-master/src/xc.h with TWO
 * conscious deviations from strict drop-in compatibility:
 *
 *   1. `xc_func_type` and `xc_func_info_type` are forward-declared opaque only.
 *      C code that accesses fields directly (e.g. `p->info`, `info->kind`)
 *      will NOT compile against this header; migrate to the accessor functions
 *      (`xc_func_get_info`, `xc_func_info_get_*`, `xc_hyb_*`, ...).
 *
 *   2. Functions that returned `void` in libxc 7.0.0 (lifecycle end, setters,
 *      evaluators) now return `int` (0 == LIBXC_RS_OK on success; a negative
 *      LIBXC_RS_* code on failure). C call sites that ignore the return value
 *      compile and run unchanged; Fortran call sites bound as `subroutine`
 *      need a one-line subroutine->function change to observe errors.
 *
 * Recover the typed error after any non-zero return:
 *     int         xc_rs_last_error_code(void);
 *     const char *xc_rs_last_error_message(void);
 *
 * THREADING: each `xc_func_type *` may be used by only one thread at a time.
 * Use multiple handles for parallel evaluation (matches libxc's de-facto rule).
 *
 * STRING LIFETIME: pointers returned by name-getter functions
 * (xc_functional_get_name, xc_func_info_get_name, ...) remain valid until the
 * thread exits. Copy immediately if you need to outlive that.
 */

#ifndef _XC_H
#define _XC_H

#include <stddef.h>     /* size_t */

#ifdef __cplusplus
extern "C" {
#endif

/* === Library version & reference === */

const char *xc_reference(void);
const char *xc_reference_doi(void);
const char *xc_reference_key(void);
void        xc_version(int *major, int *minor, int *micro);
const char *xc_version_string(void);

/* === Public constants (values match libxc 7.0.0 byte-for-byte) === */

#define XC_UNPOLARIZED          1
#define XC_POLARIZED            2

#define XC_FAMILY_LDA           1
#define XC_FAMILY_GGA           2
#define XC_FAMILY_MGGA          4
/* (1D / 2D / OEP / LCA family flags omitted: out of scope per PROJECT.md) */

#define XC_EXCHANGE             0
#define XC_CORRELATION          1
#define XC_EXCHANGE_CORRELATION 2
#define XC_KINETIC              3

/* Derivative-availability + input-requirement flags (subset of libxc's set;
 * match src/model/mod.rs::FunctionalFlags bit positions). */
#define XC_FLAGS_HAVE_EXC       (1 <<  0)
#define XC_FLAGS_HAVE_VXC       (1 <<  1)
#define XC_FLAGS_HAVE_FXC       (1 <<  2)
#define XC_FLAGS_HAVE_KXC       (1 <<  3)
#define XC_FLAGS_HAVE_LXC       (1 <<  4)
#define XC_FLAGS_NEEDS_LAPLACIAN (1 << 15)
#define XC_FLAGS_NEEDS_TAU      (1 << 16)

#define XC_HYB_NONE             0
#define XC_HYB_SEMILOCAL        0
#define XC_HYB_HYBRID           1
#define XC_HYB_CAM              2
#define XC_HYB_CAMY             3
#define XC_HYB_CAMG             4
#define XC_HYB_DOUBLE_HYBRID    5
#define XC_HYB_MIXTURE          32768

#define XC_MAX_REFERENCES       5
#define XC_EXT_PARAMS_DEFAULT   (-999998888.0)

/* === libxc_rs-specific error codes (see xc_rs_last_error_code) ===
 * Mirrors src/compat/errno.rs verbatim — 25 codes (LIBXC_RS_OK + 24 negatives).
 * Every LibxcRsError variant has a unique code; there is no catch-all. */

#define LIBXC_RS_OK                            0
#define LIBXC_RS_PANIC                        -1
#define LIBXC_RS_NULL_HANDLE                  -2
#define LIBXC_RS_UNINITIALIZED_HANDLE         -3
#define LIBXC_RS_UNKNOWN_FUNCTIONAL_ID        -4
#define LIBXC_RS_UNKNOWN_FUNCTIONAL_NAME      -5
#define LIBXC_RS_REMOVED_FUNCTIONAL_ID        -6
#define LIBXC_RS_UNKNOWN_EXT_PARAM_NAME       -7
#define LIBXC_RS_EXT_PARAM_INDEX_OUT_OF_RANGE -8
#define LIBXC_RS_EXT_PARAM_COUNT_MISMATCH     -9
#define LIBXC_RS_FAMILY_MISMATCH              -10
#define LIBXC_RS_SPIN_MISMATCH                -11
#define LIBXC_RS_INPUT_BUFFER_SIZE_MISMATCH   -12
#define LIBXC_RS_OUTPUT_BUFFER_SIZE_MISMATCH  -13
#define LIBXC_RS_BATCH_OVERFLOW               -14
#define LIBXC_RS_UNSUPPORTED_DERIVATIVE_ORDER -15
#define LIBXC_RS_UNSUPPORTED_FUNCTIONAL       -16
#define LIBXC_RS_EXT_PARAM_NOT_FOUND          -17
#define LIBXC_RS_GPU_NOT_AVAILABLE            -18
#define LIBXC_RS_DEVICE_CAPABILITY_MISMATCH   -19
#define LIBXC_RS_ALL_BELOW_THRESHOLD          -20
#define LIBXC_RS_WORKSPACE_MISMATCH           -21
#define LIBXC_RS_KERNEL_LAUNCH_FAILED         -22
#define LIBXC_RS_AUXILIARY_INIT_FAILED        -23
#define LIBXC_RS_PROPAGATION_CONFLICT         -24
#define LIBXC_RS_INVALID_SPIN                 -25

/* === Opaque types (forward declarations only — see deviation #1) === */

typedef struct xc_func_type xc_func_type;
typedef struct xc_func_info_type xc_func_info_type;
typedef struct func_reference_type func_reference_type;

/* === xc_*_out_params structs (used by xc_lda_new, xc_gga_new) === */

typedef struct {
  double *zk, *vrho, *v2rho2, *v3rho3, *v4rho4;
} xc_lda_out_params;

typedef struct {
  double *zk;
  double *vrho, *vsigma;
  double *v2rho2, *v2rhosigma, *v2sigma2;
  double *v3rho3, *v3rho2sigma, *v3rhosigma2, *v3sigma3;
  double *v4rho4, *v4rho3sigma, *v4rho2sigma2, *v4rhosigma3, *v4sigma4;
} xc_gga_out_params;

/* (xc_mgga_out_params not exposed: no xc_mgga_new in the public ABI.) */

/* === Discovery (8) === */

int         xc_functional_get_number(const char *name);
const char *xc_functional_get_name(int number);    /* thread-local string lifetime */
int         xc_family_from_id(int id, int *family, int *number);
int         xc_number_of_functionals(void);
int         xc_maximum_name_length(void);
void        xc_available_functional_numbers(int *list);
void        xc_available_functional_numbers_by_name(int *list);
void        xc_available_functional_names(char **list); /* thread-local string lifetime */

/* === Lifecycle (5) === */

xc_func_type *xc_func_alloc(void);
int           xc_func_init(xc_func_type *p, int functional, int nspin);
int           xc_func_end(xc_func_type *p);   /* libxc returned void; we return int */
void          xc_func_free(xc_func_type *p);
const xc_func_info_type *xc_func_get_info(const xc_func_type *p);

/* === Threshold setters (4) — return int per deviation #2 === */

int xc_func_set_dens_threshold(xc_func_type *p, double t_dens);
int xc_func_set_zeta_threshold(xc_func_type *p, double t_zeta);
int xc_func_set_sigma_threshold(xc_func_type *p, double t_sigma);
int xc_func_set_tau_threshold(xc_func_type *p, double t_tau);

/* === External parameters (5) === */

int    xc_func_set_ext_params(xc_func_type *p, const double *ext_params);
int    xc_func_get_ext_params(const xc_func_type *p, double *ext_params);
int    xc_func_set_ext_params_name(xc_func_type *p, const char *name, double par);
double xc_func_get_ext_params_name(const xc_func_type *p, const char *name);
double xc_func_get_ext_params_value(const xc_func_type *p, int number);

/* === Info accessors (10) === */

int         xc_func_info_get_number(const xc_func_info_type *info);
int         xc_func_info_get_kind(const xc_func_info_type *info);
const char *xc_func_info_get_name(const xc_func_info_type *info);
int         xc_func_info_get_family(const xc_func_info_type *info);
int         xc_func_info_get_flags(const xc_func_info_type *info);
int         xc_func_info_get_n_ext_params(const xc_func_info_type *info);
const char *xc_func_info_get_ext_params_name(const xc_func_info_type *info, int number);
const char *xc_func_info_get_ext_params_description(const xc_func_info_type *info, int number);
double      xc_func_info_get_ext_params_default_value(const xc_func_info_type *info, int number);
const func_reference_type *xc_func_info_get_references(const xc_func_info_type *info, int number);

/* === Reference accessors (4) === */

const char *xc_func_reference_get_ref(const func_reference_type *reference);
const char *xc_func_reference_get_doi(const func_reference_type *reference);
const char *xc_func_reference_get_bibtex(const func_reference_type *reference);
const char *xc_func_reference_get_key(const func_reference_type *reference);

/* === Hybrid / aux / NLC (7) === */

int    xc_hyb_type(const xc_func_type *p);
double xc_hyb_exx_coef(const xc_func_type *p);
void   xc_hyb_cam_coef(const xc_func_type *p, double *omega, double *alpha, double *beta);
void   xc_nlc_coef(const xc_func_type *p, double *nlc_b, double *nlc_C);
int    xc_num_aux_funcs(const xc_func_type *p);
void   xc_aux_func_ids(const xc_func_type *p, int *ids);
void   xc_aux_func_weights(const xc_func_type *p, double *weights);

/* === GGA AK13 helpers (2) === */

double xc_gga_ak13_get_asymptotic(double homo);
double xc_gga_ak13_pars_get_asymptotic(double homo, const double *ext_params);

/* === LDA evaluate (12) — return int per deviation #2 === */

int xc_lda_new(const xc_func_type *p, int order, size_t np,
               const double *rho, xc_lda_out_params *out);
int xc_lda(const xc_func_type *p, size_t np, const double *rho,
           double *zk, double *vrho, double *v2rho2, double *v3rho3, double *v4rho4);
int xc_lda_exc(const xc_func_type *p, size_t np, const double *rho, double *zk);
int xc_lda_exc_vxc(const xc_func_type *p, size_t np, const double *rho, double *zk, double *vrho);
int xc_lda_vxc(const xc_func_type *p, size_t np, const double *rho, double *vrho);
int xc_lda_exc_vxc_fxc(const xc_func_type *p, size_t np, const double *rho, double *zk, double *vrho, double *v2rho2);
int xc_lda_vxc_fxc(const xc_func_type *p, size_t np, const double *rho, double *vrho, double *v2rho2);
int xc_lda_fxc(const xc_func_type *p, size_t np, const double *rho, double *v2rho2);
int xc_lda_exc_vxc_fxc_kxc(const xc_func_type *p, size_t np, const double *rho, double *zk, double *vrho, double *v2rho2, double *v3rho3);
int xc_lda_vxc_fxc_kxc(const xc_func_type *p, size_t np, const double *rho, double *vrho, double *v2rho2, double *v3rho3);
int xc_lda_kxc(const xc_func_type *p, size_t np, const double *rho, double *v3rho3);
int xc_lda_lxc(const xc_func_type *p, size_t np, const double *rho, double *v4rho4);

/* === GGA evaluate (12) === */

int xc_gga_new(const xc_func_type *p, int order, size_t np,
               const double *rho, const double *sigma, xc_gga_out_params *out);
int xc_gga(const xc_func_type *p, size_t np, const double *rho, const double *sigma,
           double *zk, double *vrho, double *vsigma,
           double *v2rho2, double *v2rhosigma, double *v2sigma2,
           double *v3rho3, double *v3rho2sigma, double *v3rhosigma2, double *v3sigma3,
           double *v4rho4, double *v4rho3sigma, double *v4rho2sigma2, double *v4rhosigma3, double *v4sigma4);
int xc_gga_exc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *zk);
int xc_gga_exc_vxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *zk, double *vrho, double *vsigma);
int xc_gga_vxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *vrho, double *vsigma);
int xc_gga_exc_vxc_fxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *zk, double *vrho, double *vsigma, double *v2rho2, double *v2rhosigma, double *v2sigma2);
int xc_gga_vxc_fxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *vrho, double *vsigma, double *v2rho2, double *v2rhosigma, double *v2sigma2);
int xc_gga_fxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *v2rho2, double *v2rhosigma, double *v2sigma2);
int xc_gga_exc_vxc_fxc_kxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *zk, double *vrho, double *vsigma, double *v2rho2, double *v2rhosigma, double *v2sigma2, double *v3rho3, double *v3rho2sigma, double *v3rhosigma2, double *v3sigma3);
int xc_gga_vxc_fxc_kxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *vrho, double *vsigma, double *v2rho2, double *v2rhosigma, double *v2sigma2, double *v3rho3, double *v3rho2sigma, double *v3rhosigma2, double *v3sigma3);
int xc_gga_kxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *v3rho3, double *v3rho2sigma, double *v3rhosigma2, double *v3sigma3);
int xc_gga_lxc(const xc_func_type *p, size_t np, const double *rho, const double *sigma, double *v4rho4, double *v4rho3sigma, double *v4rho2sigma2, double *v4rhosigma3, double *v4sigma4);

/* === MGGA evaluate (11) — full pointer lists mirror libxc-master/src/xc.h:436-580 ===
 * Argument order: (rho, sigma, lapl, tau) inputs, then derivative outputs in
 * derivative-class-then-position order. No xc_mgga_new in the public ABI. */

int xc_mgga(const xc_func_type *p, size_t np,
            const double *rho, const double *sigma, const double *lapl, const double *tau,
            double *zk, double *vrho, double *vsigma, double *vlapl, double *vtau,
            double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
            double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
            double *v2lapltau, double *v2tau2,
            double *v3rho3, double *v3rho2sigma, double *v3rho2lapl, double *v3rho2tau,
            double *v3rhosigma2, double *v3rhosigmalapl, double *v3rhosigmatau,
            double *v3rholapl2, double *v3rholapltau, double *v3rhotau2, double *v3sigma3,
            double *v3sigma2lapl, double *v3sigma2tau, double *v3sigmalapl2, double *v3sigmalapltau,
            double *v3sigmatau2, double *v3lapl3, double *v3lapl2tau, double *v3lapltau2,
            double *v3tau3,
            double *v4rho4, double *v4rho3sigma, double *v4rho3lapl, double *v4rho3tau, double *v4rho2sigma2,
            double *v4rho2sigmalapl, double *v4rho2sigmatau, double *v4rho2lapl2, double *v4rho2lapltau,
            double *v4rho2tau2, double *v4rhosigma3, double *v4rhosigma2lapl, double *v4rhosigma2tau,
            double *v4rhosigmalapl2, double *v4rhosigmalapltau, double *v4rhosigmatau2,
            double *v4rholapl3, double *v4rholapl2tau, double *v4rholapltau2, double *v4rhotau3,
            double *v4sigma4, double *v4sigma3lapl, double *v4sigma3tau, double *v4sigma2lapl2,
            double *v4sigma2lapltau, double *v4sigma2tau2, double *v4sigmalapl3, double *v4sigmalapl2tau,
            double *v4sigmalapltau2, double *v4sigmatau3, double *v4lapl4, double *v4lapl3tau,
            double *v4lapl2tau2, double *v4lapltau3, double *v4tau4);
int xc_mgga_exc(const xc_func_type *p, size_t np,
                const double *rho, const double *sigma, const double *lapl, const double *tau,
                double *zk);
int xc_mgga_exc_vxc(const xc_func_type *p, size_t np,
                    const double *rho, const double *sigma, const double *lapl, const double *tau,
                    double *zk, double *vrho, double *vsigma, double *vlapl, double *vtau);
int xc_mgga_vxc(const xc_func_type *p, size_t np,
                const double *rho, const double *sigma, const double *lapl, const double *tau,
                double *vrho, double *vsigma, double *vlapl, double *vtau);
int xc_mgga_exc_vxc_fxc(const xc_func_type *p, size_t np,
                        const double *rho, const double *sigma, const double *lapl, const double *tau,
                        double *zk, double *vrho, double *vsigma, double *vlapl, double *vtau,
                        double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
                        double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
                        double *v2lapltau, double *v2tau2);
int xc_mgga_vxc_fxc(const xc_func_type *p, size_t np,
                    const double *rho, const double *sigma, const double *lapl, const double *tau,
                    double *vrho, double *vsigma, double *vlapl, double *vtau,
                    double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
                    double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
                    double *v2lapltau, double *v2tau2);
int xc_mgga_fxc(const xc_func_type *p, size_t np,
                const double *rho, const double *sigma, const double *lapl, const double *tau,
                double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
                double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
                double *v2lapltau, double *v2tau2);
int xc_mgga_exc_vxc_fxc_kxc(const xc_func_type *p, size_t np,
                            const double *rho, const double *sigma, const double *lapl, const double *tau,
                            double *zk, double *vrho, double *vsigma, double *vlapl, double *vtau,
                            double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
                            double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
                            double *v2lapltau, double *v2tau2,
                            double *v3rho3, double *v3rho2sigma, double *v3rho2lapl, double *v3rho2tau,
                            double *v3rhosigma2, double *v3rhosigmalapl, double *v3rhosigmatau,
                            double *v3rholapl2, double *v3rholapltau, double *v3rhotau2, double *v3sigma3,
                            double *v3sigma2lapl, double *v3sigma2tau, double *v3sigmalapl2, double *v3sigmalapltau,
                            double *v3sigmatau2, double *v3lapl3, double *v3lapl2tau, double *v3lapltau2,
                            double *v3tau3);
int xc_mgga_vxc_fxc_kxc(const xc_func_type *p, size_t np,
                        const double *rho, const double *sigma, const double *lapl, const double *tau,
                        double *vrho, double *vsigma, double *vlapl, double *vtau,
                        double *v2rho2, double *v2rhosigma, double *v2rholapl, double *v2rhotau,
                        double *v2sigma2, double *v2sigmalapl, double *v2sigmatau, double *v2lapl2,
                        double *v2lapltau, double *v2tau2,
                        double *v3rho3, double *v3rho2sigma, double *v3rho2lapl, double *v3rho2tau,
                        double *v3rhosigma2, double *v3rhosigmalapl, double *v3rhosigmatau,
                        double *v3rholapl2, double *v3rholapltau, double *v3rhotau2, double *v3sigma3,
                        double *v3sigma2lapl, double *v3sigma2tau, double *v3sigmalapl2, double *v3sigmalapltau,
                        double *v3sigmatau2, double *v3lapl3, double *v3lapl2tau, double *v3lapltau2,
                        double *v3tau3);
int xc_mgga_kxc(const xc_func_type *p, size_t np,
                const double *rho, const double *sigma, const double *lapl, const double *tau,
                double *v3rho3, double *v3rho2sigma, double *v3rho2lapl, double *v3rho2tau,
                double *v3rhosigma2, double *v3rhosigmalapl, double *v3rhosigmatau,
                double *v3rholapl2, double *v3rholapltau, double *v3rhotau2, double *v3sigma3,
                double *v3sigma2lapl, double *v3sigma2tau, double *v3sigmalapl2, double *v3sigmalapltau,
                double *v3sigmatau2, double *v3lapl3, double *v3lapl2tau, double *v3lapltau2,
                double *v3tau3);
int xc_mgga_lxc(const xc_func_type *p, size_t np,
                const double *rho, const double *sigma, const double *lapl, const double *tau,
                double *v4rho4, double *v4rho3sigma, double *v4rho3lapl, double *v4rho3tau, double *v4rho2sigma2,
                double *v4rho2sigmalapl, double *v4rho2sigmatau, double *v4rho2lapl2, double *v4rho2lapltau,
                double *v4rho2tau2, double *v4rhosigma3, double *v4rhosigma2lapl, double *v4rhosigma2tau,
                double *v4rhosigmalapl2, double *v4rhosigmalapltau, double *v4rhosigmatau2,
                double *v4rholapl3, double *v4rholapl2tau, double *v4rholapltau2, double *v4rhotau3,
                double *v4sigma4, double *v4sigma3lapl, double *v4sigma3tau, double *v4sigma2lapl2,
                double *v4sigma2lapltau, double *v4sigma2tau2, double *v4sigmalapl3, double *v4sigmalapl2tau,
                double *v4sigmalapltau2, double *v4sigmatau3, double *v4lapl4, double *v4lapl3tau,
                double *v4lapl2tau2, double *v4lapltau3, double *v4tau4);

/* === libxc_rs-specific errno accessors (2) === */

/* Most recent error code on this thread, or LIBXC_RS_OK if none. */
int         xc_rs_last_error_code(void);
/* Most recent error message on this thread. Valid until the next error-setting
 * call on this thread. Never NULL — empty string when no error recorded. */
const char *xc_rs_last_error_message(void);

#ifdef __cplusplus
}
#endif

#endif  /* _XC_H */
