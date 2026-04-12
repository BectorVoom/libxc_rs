//! GGA_K_OL2 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol2_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_bb * sigma[ip];
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = t26 * t29;
        let t33 = f64::sqrt(sigma[ip]);
        let t34 = param_cc * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = 4.0 * t33 * t25 * t36 + t25;
        let t42 = 1.0 / t41;
        let t43 = t25 * t36 * t42;
        let t45 = param_aa + 0.13888888888888888889e-1 * t24 * t30 + t34 * t43;
        let t49 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t51 = t20 / t21;
        let t55 = t27 * rho[ip];
        let t57 = 1.0 / t22 / t55;
        let t58 = t26 * t57;
        let t62 = 1.0 / t21 / t27;
        let t64 = t25 * t62 * t42;
        let t67 = param_cc * sigma[ip];
        let t68 = t41 * t41;
        let t69 = 1.0 / t68;
        let t70 = t58 * t69;
        let t73 = -0.37037037037037037037e-1 * t24 * t58 - 4.0 / 3.0 * t34 * t64 + 16.0 / 3.0 * t67 * t70;
        let t78 = piecewise3(t2, 0.0, t7 * t51 * t45 / 10.0 + 3.0 / 20.0 * t7 * t23 * t73);
        let tvrho0 = 2.0 * rho[ip] * t78 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t81 = param_bb * t26;
        let t84 = 1.0 / t33;
        let t85 = param_cc * t84;
        let t88 = param_cc * t26;
        let t92 = 0.13888888888888888889e-1 * t81 * t29 + t85 * t43 / 2.0 - 2.0 * t88 * t29 * t69;
        let t96 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t92);
        let tvsigma0 = 2.0 * rho[ip] * t96;
        vsigma[ip] += tvsigma0;
        let t99 = t20 * t36;
        let t106 = t27 * t27;
        let t108 = 1.0 / t22 / t106;
        let t109 = t26 * t108;
        let t113 = 1.0 / t21 / t55;
        let t115 = t25 * t113 * t42;
        let t118 = t109 * t69;
        let t121 = t33 * sigma[ip];
        let t122 = param_cc * t121;
        let t123 = t106 * t27;
        let t124 = 1.0 / t123;
        let t126 = 1.0 / t68 / t41;
        let t127 = t124 * t126;
        let t130 = 0.13580246913580246914e0 * t24 * t109 + 28.0 / 9.0 * t34 * t115 - 80.0 / 3.0 * t67 * t118 + 1024.0 / 9.0 * t122 * t127;
        let t135 = piecewise3(t2, 0.0, -t7 * t99 * t45 / 30.0 + t7 * t51 * t73 / 5.0 + 3.0 / 20.0 * t7 * t23 * t130);
        let tv2rho20 = 2.0 * rho[ip] * t135 + 4.0 * t78;
        v2rho2[ip] += tv2rho20;
        let t148 = t106 * rho[ip];
        let t149 = 1.0 / t148;
        let t151 = t126 * t33;
        let t154 = -0.37037037037037037037e-1 * t81 * t57 - 2.0 / 3.0 * t85 * t64 + 8.0 * t88 * t57 * t69 - 128.0 / 3.0 * param_cc * t149 * t151;
        let t159 = piecewise3(t2, 0.0, t7 * t51 * t92 / 10.0 + 3.0 / 20.0 * t7 * t23 * t154);
        let tv2rhosigma0 = 2.0 * rho[ip] * t159 + 2.0 * t96;
        v2rhosigma[ip] += tv2rhosigma0;
        let t162 = 1.0 / t121;
        let t163 = param_cc * t162;
        let t166 = 1.0 / sigma[ip];
        let t167 = param_cc * t166;
        let t168 = t30 * t69;
        let t170 = 1.0 / t106;
        let t175 = -t163 * t43 / 4.0 - t167 * t168 + 16.0 * param_cc * t170 * t126 * t84;
        let t179 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t175);
        let tv2sigma20 = 2.0 * rho[ip] * t179;
        v2sigma2[ip] += tv2sigma20;
        let t182 = t20 * t62;
        let t193 = 1.0 / t22 / t148;
        let t194 = t26 * t193;
        let t200 = t25 / t21 / t106 * t42;
        let t206 = t106 * t55;
        let t207 = 1.0 / t206;
        let t211 = sigma[ip] * sigma[ip];
        let t212 = param_cc * t211;
        let t213 = t106 * t106;
        let t215 = 1.0 / t21 / t213;
        let t216 = t68 * t68;
        let t217 = 1.0 / t216;
        let t222 = -0.63374485596707818932e0 * t24 * t194 - 280.0 / 27.0 * t34 * t200 + 3808.0 / 27.0 * t67 * t194 * t69 - 11264.0 / 9.0 * t122 * t207 * t126 + 16384.0 / 9.0 * t212 * t215 * t217 * t25;
        let t227 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t182 * t45 - t7 * t99 * t73 / 10.0 + 3.0 / 10.0 * t7 * t51 * t130 + 3.0 / 20.0 * t7 * t23 * t222);
        let tv3rho30 = 2.0 * rho[ip] * t227 + 6.0 * t135;
        v3rho3[ip] += tv3rho30;
        let t249 = param_cc / t21 / t206;
        let t251 = t217 * sigma[ip] * t25;
        let t254 = 0.13580246913580246914e0 * t81 * t108 + 14.0 / 9.0 * t85 * t115 - 296.0 / 9.0 * t88 * t108 * t69 + 384.0 * param_cc * t124 * t151 - 2048.0 / 3.0 * t249 * t251;
        let t259 = piecewise3(t2, 0.0, -t7 * t99 * t92 / 30.0 + t7 * t51 * t154 / 5.0 + 3.0 / 20.0 * t7 * t23 * t254);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t259 + 4.0 * t159;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t269 = t149 * t126;
        let t274 = param_cc / t21 / t123;
        let t275 = t217 * t25;
        let t278 = t163 * t64 / 3.0 + 4.0 / 3.0 * t167 * t70 - 256.0 / 3.0 * t85 * t269 + 256.0 * t274 * t275;
        let t283 = piecewise3(t2, 0.0, t7 * t51 * t175 / 10.0 + 3.0 / 20.0 * t7 * t23 * t278);
        let tv3rhosigma20 = 2.0 * rho[ip] * t283 + 2.0 * t179;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t286 = t33 * t211;
        let t288 = param_cc / t286;
        let t291 = 1.0 / t211;
        let t292 = param_cc * t291;
        let t296 = 1.0 / t21 / t148;
        let t297 = param_cc * t296;
        let t299 = t217 * t166 * t25;
        let t302 = 3.0 / 8.0 * t288 * t43 + 3.0 / 2.0 * t292 * t168 - 96.0 * t297 * t299;
        let t306 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t302);
        let tv3sigma30 = 2.0 * rho[ip] * t306;
        v3sigma3[ip] += tv3sigma30;
        let t323 = 1.0 / t22 / t123;
        let t324 = t26 * t323;
        let t338 = t213 * rho[ip];
        let t350 = 1.0 / t216 / t41;
        let t360 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 * t113 * t45 + 8.0 / 45.0 * t7 * t182 * t73 - t7 * t99 * t130 / 5.0 + 2.0 / 5.0 * t7 * t51 * t222 + 3.0 / 20.0 * t7 * t23 * (0.35912208504801097395e1 * t24 * t324 + 3640.0 / 81.0 * t34 * t25 * t296 * t42 - 23072.0 / 27.0 * t67 * t324 * t69 + 953344.0 / 81.0 * t122 / t213 * t126 - 950272.0 / 27.0 * t212 / t21 / t338 * t217 * t25 + 1048576.0 / 27.0 * param_cc * t286 / t22 / t213 / t27 * t350 * t26));
        let tv4rho40 = 2.0 * rho[ip] * t360 + 8.0 * t227;
        v4rho4[ip] += tv4rho40;
        let t398 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t182 * t92 - t7 * t99 * t154 / 10.0 + 3.0 / 10.0 * t7 * t51 * t254 + 3.0 / 20.0 * t7 * t23 * (-0.63374485596707818932e0 * t81 * t193 - 140.0 / 27.0 * t85 * t200 + 1456.0 / 9.0 * t88 * t193 * t69 - 81152.0 / 27.0 * param_cc * t207 * t151 + 100352.0 / 9.0 * param_cc * t215 * t251 - 131072.0 / 9.0 * param_cc / t22 / t338 * t350 * t121 * t26));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t398 + 6.0 * t259;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t428 = piecewise3(t2, 0.0, -t7 * t99 * t175 / 30.0 + t7 * t51 * t278 / 5.0 + 3.0 / 20.0 * t7 * t23 * (-7.0 / 9.0 * t163 * t115 - 28.0 / 9.0 * t167 * t118 + 4096.0 / 9.0 * t85 * t127 - 8960.0 / 3.0 * t249 * t275 + 16384.0 / 3.0 * param_cc / t22 / t213 * t350 * t26 * t33));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t428 + 4.0 * t283;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t454 = piecewise3(t2, 0.0, t7 * t51 * t302 / 10.0 + 3.0 / 20.0 * t7 * t23 * (-t288 * t64 / 2.0 - 2.0 * t292 * t70 + 32.0 * t163 * t269 + 512.0 * t274 * t299 - 2048.0 * param_cc / t22 / t206 * t350 * t84 * t26));
        let tv4rhosigma30 = 2.0 * rho[ip] * t454 + 2.0 * t306;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t457 = t211 * sigma[ip];
        let t483 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (-15.0 / 16.0 * param_cc / t33 / t457 * t43 - 15.0 / 4.0 * param_cc / t457 * t168 - 12.0 * t288 * t170 * t126 + 768.0 * param_cc * t323 * t350 * t162 * t26 + 96.0 * t297 * t217 * t291 * t25));
        let tv4sigma40 = 2.0 * rho[ip] * t483;
        v4sigma4[ip] += tv4sigma40;
    }
}
