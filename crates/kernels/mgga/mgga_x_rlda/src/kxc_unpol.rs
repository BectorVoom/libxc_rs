//! MGGA_X_RLDA kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_rlda_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t21 = pow_1_3(1.0 / M_PI);
        let t22 = 1.0 / t21;
        let t23 = param_prefactor * t22;
        let t24 = M_CBRT4;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t18 * t18;
        let t30 = 1.0 / t28 / rho[ip];
        let t33 = lapl[ip] * t26;
        let t36 = 2.0 * t27 * t30 - t33 * t30 / 4.0;
        let t39 = t23 * t24 / t36;
        let t42 = piecewise3(t3, 0.0, -15.0 / 16.0 * t17 * t18 * t39);
        let tzk0 = 2.0 * t42;
        zk[ip] += tzk0;
        let t43 = 1.0 / t28;
        let t48 = t17 * t18 * param_prefactor;
        let t49 = t22 * t24;
        let t50 = t36 * t36;
        let t51 = 1.0 / t50;
        let t52 = rho[ip] * rho[ip];
        let t54 = 1.0 / t28 / t52;
        let t59 = -10.0 / 3.0 * t27 * t54 + 5.0 / 12.0 * t33 * t54;
        let t61 = t49 * t51 * t59;
        let t65 = piecewise3(t3, 0.0, -5.0 / 16.0 * t17 * t43 * t39 + 15.0 / 16.0 * t48 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t42;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t70 = 1.0 / t18 / rho[ip] * param_prefactor;
        let t71 = t17 * t70;
        let t73 = t49 * t51 * t26;
        let t74 = t71 * t73;
        let t76 = piecewise3(t3, 0.0, -15.0 / 64.0 * t74);
        let tvlapl0 = 2.0 * rho[ip] * t76;
        vlapl[ip] += tvlapl0;
        let t79 = piecewise3(t3, 0.0, 15.0 / 8.0 * t74);
        let tvtau0 = 2.0 * rho[ip] * t79;
        vtau[ip] += tvtau0;
        let t86 = t17 * t43 * param_prefactor;
        let t90 = 1.0 / t50 / t36;
        let t91 = t59 * t59;
        let t93 = t49 * t90 * t91;
        let t96 = t52 * rho[ip];
        let t98 = 1.0 / t28 / t96;
        let t103 = 80.0 / 9.0 * t27 * t98 - 10.0 / 9.0 * t33 * t98;
        let t105 = t49 * t51 * t103;
        let t109 = piecewise3(t3, 0.0, 5.0 / 24.0 * t17 * t30 * t39 + 5.0 / 8.0 * t86 * t61 - 15.0 / 8.0 * t48 * t93 + 15.0 / 16.0 * t48 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t109 + 4.0 * t65;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t115 = t17 / t18 / t52 * param_prefactor;
        let t116 = t115 * t73;
        let t118 = t90 * t26;
        let t120 = t49 * t118 * t59;
        let t121 = t71 * t120;
        let t124 = piecewise3(t3, 0.0, 5.0 / 16.0 * t116 + 15.0 / 32.0 * t121);
        let tv2rholapl0 = 2.0 * rho[ip] * t124 + 2.0 * t76;
        v2rholapl[ip] += tv2rholapl0;
        let t130 = piecewise3(t3, 0.0, -5.0 / 2.0 * t116 - 15.0 / 4.0 * t121);
        let tv2rhotau0 = 2.0 * rho[ip] * t130 + 2.0 * t79;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t135 = t17 / t96 * param_prefactor;
        let t137 = t49 * t90 * t25;
        let t138 = t135 * t137;
        let t140 = piecewise3(t3, 0.0, -15.0 / 64.0 * t138);
        let tv2lapl20 = 2.0 * rho[ip] * t140;
        v2lapl2[ip] += tv2lapl20;
        let t143 = piecewise3(t3, 0.0, 15.0 / 8.0 * t138);
        let tv2lapltau0 = 2.0 * rho[ip] * t143;
        v2lapltau[ip] += tv2lapltau0;
        let t146 = piecewise3(t3, 0.0, -15.0 * t138);
        let tv2tau20 = 2.0 * rho[ip] * t146;
        v2tau2[ip] += tv2tau20;
        let t153 = t17 * t30 * param_prefactor;
        let t160 = t50 * t50;
        let t161 = 1.0 / t160;
        let t162 = t91 * t59;
        let t164 = t49 * t161 * t162;
        let t167 = t90 * t59;
        let t169 = t49 * t167 * t103;
        let t172 = t52 * t52;
        let t174 = 1.0 / t28 / t172;
        let t179 = -880.0 / 27.0 * t27 * t174 + 110.0 / 27.0 * t33 * t174;
        let t181 = t49 * t51 * t179;
        let t185 = piecewise3(t3, 0.0, -25.0 / 72.0 * t17 * t54 * t39 - 5.0 / 8.0 * t153 * t61 - 15.0 / 8.0 * t86 * t93 + 15.0 / 16.0 * t86 * t105 + 45.0 / 8.0 * t48 * t164 - 45.0 / 8.0 * t48 * t169 + 15.0 / 16.0 * t48 * t181);
        let tv3rho30 = 2.0 * rho[ip] * t185 + 6.0 * t109;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t192 = t17 / t18 / t96 * param_prefactor;
        let t193 = t192 * t73;
        let t195 = t115 * t120;
        let t199 = t49 * t161 * t26 * t91;
        let t200 = t71 * t199;
        let t203 = t49 * t118 * t103;
        let t204 = t71 * t203;
        let t207 = piecewise3(t3, 0.0, -35.0 / 48.0 * t193 - 5.0 / 4.0 * t195 - 45.0 / 32.0 * t200 + 15.0 / 32.0 * t204);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t207 + 4.0 * t124;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t216 = piecewise3(t3, 0.0, 35.0 / 6.0 * t193 + 10.0 * t195 + 45.0 / 4.0 * t200 - 15.0 / 4.0 * t204);
        let tv3rho2tau0 = 2.0 * rho[ip] * t216 + 4.0 * t130;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t221 = t17 / t172 * param_prefactor;
        let t223 = t161 * t25;
        let t225 = t49 * t223 * t59;
        let t227 = t135 * t225 + t221 * t137;
        let t229 = piecewise3(t3, 0.0, 45.0 / 64.0 * t227);
        let tv3rholapl20 = 2.0 * rho[ip] * t229 + 2.0 * t140;
        v3rholapl2[ip] += tv3rholapl20;
        let t234 = piecewise3(t3, 0.0, -45.0 / 8.0 * t227);
        let tv3rholapltau0 = 2.0 * rho[ip] * t234 + 2.0 * t143;
        v3rholapltau[ip] += tv3rholapltau0;
        let t238 = piecewise3(t3, 0.0, 45.0 * t227);
        let tv3rhotau20 = 2.0 * rho[ip] * t238 + 2.0 * t146;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let t242 = t24 * t161;
        let t243 = t23 * t242;
        let t244 = t17 * t174 * t243;
        let t246 = piecewise3(t3, 0.0, -45.0 / 128.0 * t244);
        let tv3lapl30 = 2.0 * rho[ip] * t246;
        v3lapl3[ip] += tv3lapl30;
        let t249 = piecewise3(t3, 0.0, 45.0 / 16.0 * t244);
        let tv3lapl2tau0 = 2.0 * rho[ip] * t249;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let t252 = piecewise3(t3, 0.0, -45.0 / 2.0 * t244);
        let tv3lapltau20 = 2.0 * rho[ip] * t252;
        v3lapltau2[ip] += tv3lapltau20;
        let t255 = piecewise3(t3, 0.0, 180.0 * t244);
        let tv3tau30 = 2.0 * rho[ip] * t255;
        v3tau3[ip] += tv3tau30;
    }
}
