//! MGGA_K_PGSLB kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_pgslb_kxc_unpol(
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
    param_pgslb_beta: f64,
    param_pgslb_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5::<f64>(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3::<f64>(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3::<f64>(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3::<f64>(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3::<f64>(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3::<f64>(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t41 = param_pgslb_mu * t25 * t29;
        let t44 = f64::exp(-t41 * t37 / 24.0);
        let t45 = t25 * t25;
        let t46 = param_pgslb_beta * t45;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = 5.0 / 72.0 * t30 * t37 + t44 + t49 * t51 * t54 / 288.0;
        let t62 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        let t64 = t21 / t22;
        let t69 = 1.0 / t23 / t52;
        let t77 = t34 * t34;
        let t83 = -5.0 / 27.0 * t30 * t33 * t69 + t41 * t33 * t69 * t44 / 9.0 - 5.0 / 432.0 * t49 * t51 / t22 / t77;
        let t88 = piecewise3::<f64>(t3, 0.0, t8 * t64 * t58 / 10.0 + 3.0 / 20.0 * t8 * t24 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t91 = t32 * t36;
        let t97 = 5.0 / 72.0 * t30 * t91 - t41 * t91 * t44 / 24.0;
        let t101 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t97);
        let tvsigma0 = 2.0 * rho[ip] * t101;
        vsigma[ip] += tvsigma0;
        let t104 = t8 * t21 * t36;
        let t107 = t46 * t48 * lapl[ip] * t31;
        let t110 = piecewise3::<f64>(t3, 0.0, t104 * t107 / 960.0);
        let tvlapl0 = 2.0 * rho[ip] * t110;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t115 = t21 / t22 / rho[ip];
        let t123 = 1.0 / t23 / t77;
        let t131 = param_pgslb_mu * param_pgslb_mu;
        let t132 = t131 * t45;
        let t133 = t132 * t48;
        let t134 = sigma[ip] * sigma[ip];
        let t135 = t134 * t31;
        let t138 = 1.0 / t22 / t77 / t52;
        let t143 = t77 * rho[ip];
        let t149 = 55.0 / 81.0 * t30 * t33 * t123 - 11.0 / 27.0 * t41 * t33 * t123 * t44 + 2.0 / 81.0 * t133 * t135 * t138 * t44 + 65.0 / 1296.0 * t49 * t51 / t22 / t143;
        let t154 = piecewise3::<f64>(t3, 0.0, -t8 * t115 * t58 / 30.0 + t8 * t64 * t83 / 5.0 + 3.0 / 20.0 * t8 * t24 * t149);
        let tv2rho20 = 2.0 * rho[ip] * t154 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t160 = t32 * t69;
        let t166 = t77 * t34;
        let t168 = 1.0 / t22 / t166;
        let t170 = sigma[ip] * t44;
        let t174 = -5.0 / 27.0 * t30 * t160 + t41 * t160 * t44 / 9.0 - t133 * t31 * t168 * t170 / 108.0;
        let t179 = piecewise3::<f64>(t3, 0.0, t8 * t64 * t97 / 10.0 + 3.0 / 20.0 * t8 * t24 * t174);
        let tv2rhosigma0 = 2.0 * rho[ip] * t179 + 2.0 * t101;
        v2rhosigma[ip] += tv2rhosigma0;
        let t183 = t8 * t21 * t69;
        let t186 = piecewise3::<f64>(t3, 0.0, -t183 * t107 / 360.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t186 + 2.0 * t110;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t190 = t8 * t21 * t123;
        let t191 = t48 * t31;
        let t193 = t132 * t191 * t44;
        let t196 = piecewise3::<f64>(t3, 0.0, t190 * t193 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t196;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t198 = t46 * t191;
        let t201 = piecewise3::<f64>(t3, 0.0, t104 * t198 / 960.0);
        let tv2lapl20 = 2.0 * rho[ip] * t201;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t206 = t21 / t22 / t34;
        let t217 = 1.0 / t23 / t143;
        let t225 = t77 * t77;
        let t227 = 1.0 / t22 / t225;
        let t232 = t131 * param_pgslb_mu;
        let t233 = t26 * t26;
        let t234 = 1.0 / t233;
        let t235 = t232 * t234;
        let t236 = t134 * sigma[ip];
        let t238 = 1.0 / t225 / t52;
        let t246 = -770.0 / 243.0 * t30 * t33 * t217 + 154.0 / 81.0 * t41 * t33 * t217 * t44 - 22.0 / 81.0 * t133 * t135 * t227 * t44 + 8.0 / 243.0 * t235 * t236 * t238 * t44 - 65.0 / 243.0 * t49 * t51 * t168;
        let t251 = piecewise3::<f64>(t3, 0.0, 2.0 / 45.0 * t8 * t206 * t58 - t8 * t115 * t83 / 10.0 + 3.0 / 10.0 * t8 * t64 * t149 + 3.0 / 20.0 * t8 * t24 * t246);
        let tv3rho30 = 2.0 * rho[ip] * t251 + 6.0 * t154;
        v3rho3[ip] += tv3rho30;
        let t261 = t32 * t123;
        let t272 = 1.0 / t225 / t34;
        let t277 = 55.0 / 81.0 * t30 * t261 - 11.0 / 27.0 * t41 * t261 * t44 + t133 * t31 * t138 * t170 / 12.0 - t235 * t272 * t134 * t44 / 81.0;
        let t282 = piecewise3::<f64>(t3, 0.0, -t8 * t115 * t97 / 30.0 + t8 * t64 * t174 / 5.0 + 3.0 / 20.0 * t8 * t24 * t277);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t282 + 4.0 * t179;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t288 = piecewise3::<f64>(t3, 0.0, 11.0 / 1080.0 * t190 * t107);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t288 + 4.0 * t186;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t292 = t8 * t21 * t217;
        let t295 = t6 * t6;
        let t298 = t5 / t295 / t26;
        let t299 = t298 * t21;
        let t300 = t227 * t232;
        let t305 = piecewise3::<f64>(t3, 0.0, -7.0 / 2880.0 * t292 * t193 + t299 * t300 * t170 / 1440.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t305 + 2.0 * t196;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t310 = piecewise3::<f64>(t3, 0.0, -t183 * t198 / 360.0);
        let tv3rholapl20 = 2.0 * rho[ip] * t310 + 2.0 * t201;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t317 = piecewise3::<f64>(t3, 0.0, -t299 * t138 * t232 * t44 / 3840.0);
        let tv3sigma30 = 2.0 * rho[ip] * t317;
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
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
    }
}
