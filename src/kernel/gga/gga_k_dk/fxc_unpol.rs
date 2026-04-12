//! GGA_K_DK fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_dk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_dk_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_aa_0: f64,
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_bb_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
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
        let t21 = t7 * t20;
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = param_aa_1;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t23 / t29;
        let t32 = t28 * t31;
        let t34 = param_aa_2;
        let t35 = sigma[ip] * sigma[ip];
        let t36 = t34 * t35;
        let t37 = t29 * t29;
        let t38 = t37 * rho[ip];
        let t40 = 1.0 / t22 / t38;
        let t41 = t27 * t40;
        let t44 = param_aa_3;
        let t45 = t35 * sigma[ip];
        let t46 = t44 * t45;
        let t47 = t37 * t37;
        let t48 = 1.0 / t47;
        let t51 = param_aa_4;
        let t52 = t35 * t35;
        let t53 = t51 * t52;
        let t54 = t47 * t29;
        let t57 = t28 / t23 / t54;
        let t60 = t26 * t32 + 2.0 * t36 * t41 + 4.0 * t46 * t48 + 4.0 * t53 * t57 + param_aa_0;
        let t61 = t23 * t60;
        let t63 = param_bb_1;
        let t64 = t63 * sigma[ip];
        let t66 = param_bb_2;
        let t67 = t66 * t35;
        let t70 = param_bb_3;
        let t71 = t70 * t45;
        let t74 = param_bb_4;
        let t75 = t74 * t52;
        let t78 = t64 * t32 + 2.0 * t67 * t41 + 4.0 * t71 * t48 + 4.0 * t75 * t57 + param_bb_0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t61 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
        let t84 = 1.0 / t22;
        let t85 = t84 * t60;
        let t89 = t29 * rho[ip];
        let t91 = 1.0 / t23 / t89;
        let t92 = t28 * t91;
        let t95 = t37 * t29;
        let t97 = 1.0 / t22 / t95;
        let t98 = t27 * t97;
        let t101 = t47 * rho[ip];
        let t102 = 1.0 / t101;
        let t105 = t47 * t89;
        let t108 = t28 / t23 / t105;
        let t111 = -8.0 / 3.0 * t26 * t92 - 32.0 / 3.0 * t36 * t98 - 32.0 * t46 * t102 - 128.0 / 3.0 * t53 * t108;
        let t112 = t23 * t111;
        let t116 = t78 * t78;
        let t117 = 1.0 / t116;
        let t126 = -8.0 / 3.0 * t64 * t92 - 32.0 / 3.0 * t67 * t98 - 32.0 * t71 * t102 - 128.0 / 3.0 * t75 * t108;
        let t127 = t117 * t126;
        let t132 = piecewise3(t2, 0.0, t21 * t85 * t79 / 10.0 + 3.0 / 20.0 * t21 * t112 * t79 - 3.0 / 20.0 * t21 * t61 * t127);
        let tvrho0 = 2.0 * rho[ip] * t132 + 2.0 * t83;
        vrho[ip] += tvrho0;
        let t135 = t25 * t28;
        let t137 = t34 * sigma[ip];
        let t140 = t44 * t35;
        let t143 = t51 * t45;
        let t146 = t135 * t31 + 4.0 * t137 * t41 + 12.0 * t140 * t48 + 16.0 * t143 * t57;
        let t147 = t23 * t146;
        let t150 = t63 * t28;
        let t152 = t66 * sigma[ip];
        let t155 = t70 * t35;
        let t158 = t74 * t45;
        let t161 = t150 * t31 + 4.0 * t152 * t41 + 12.0 * t155 * t48 + 16.0 * t158 * t57;
        let t162 = t117 * t161;
        let t167 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t147 * t79 - 3.0 / 20.0 * t21 * t61 * t162);
        let tvsigma0 = 2.0 * rho[ip] * t167;
        vsigma[ip] += tvsigma0;
        let t171 = 1.0 / t22 / rho[ip];
        let t172 = t171 * t60;
        let t176 = t84 * t111;
        let t184 = 1.0 / t23 / t37;
        let t185 = t28 * t184;
        let t190 = 1.0 / t22 / t37 / t89;
        let t191 = t27 * t190;
        let t194 = 1.0 / t54;
        let t197 = t47 * t37;
        let t200 = t28 / t23 / t197;
        let t203 = 88.0 / 9.0 * t26 * t185 + 608.0 / 9.0 * t36 * t191 + 288.0 * t46 * t194 + 4480.0 / 9.0 * t53 * t200;
        let t204 = t23 * t203;
        let t212 = 1.0 / t116 / t78;
        let t213 = t126 * t126;
        let t214 = t212 * t213;
        let t226 = 88.0 / 9.0 * t64 * t185 + 608.0 / 9.0 * t67 * t191 + 288.0 * t71 * t194 + 4480.0 / 9.0 * t75 * t200;
        let t227 = t117 * t226;
        let t232 = piecewise3(t2, 0.0, -t21 * t172 * t79 / 30.0 + t21 * t176 * t79 / 5.0 - t21 * t85 * t127 / 5.0 + 3.0 / 20.0 * t21 * t204 * t79 - 3.0 / 10.0 * t21 * t112 * t127 + 3.0 / 10.0 * t21 * t61 * t214 - 3.0 / 20.0 * t21 * t61 * t227);
        let tv2rho20 = 2.0 * rho[ip] * t232 + 4.0 * t132;
        v2rho2[ip] += tv2rho20;
        let t235 = t84 * t146;
        let t247 = -8.0 / 3.0 * t135 * t91 - 64.0 / 3.0 * t137 * t98 - 96.0 * t140 * t102 - 512.0 / 3.0 * t143 * t108;
        let t248 = t23 * t247;
        let t262 = t7 * t20 * t23;
        let t263 = t60 * t212;
        let t264 = t161 * t126;
        let t265 = t263 * t264;
        let t276 = -8.0 / 3.0 * t150 * t91 - 64.0 / 3.0 * t152 * t98 - 96.0 * t155 * t102 - 512.0 / 3.0 * t158 * t108;
        let t277 = t117 * t276;
        let t282 = piecewise3(t2, 0.0, t21 * t235 * t79 / 10.0 + 3.0 / 20.0 * t21 * t248 * t79 - 3.0 / 20.0 * t21 * t147 * t127 - t21 * t85 * t162 / 10.0 - 3.0 / 20.0 * t21 * t112 * t162 + 3.0 / 10.0 * t262 * t265 - 3.0 / 20.0 * t21 * t61 * t277);
        let tv2rhosigma0 = 2.0 * rho[ip] * t282 + 2.0 * t167;
        v2rhosigma[ip] += tv2rhosigma0;
        let t285 = t34 * t27;
        let t288 = t44 * sigma[ip];
        let t291 = t51 * t35;
        let t294 = 4.0 * t285 * t40 + 24.0 * t288 * t48 + 48.0 * t291 * t57;
        let t295 = t23 * t294;
        let t302 = t161 * t161;
        let t303 = t212 * t302;
        let t307 = t66 * t27;
        let t310 = t70 * sigma[ip];
        let t313 = t74 * t35;
        let t316 = 4.0 * t307 * t40 + 24.0 * t310 * t48 + 48.0 * t313 * t57;
        let t317 = t117 * t316;
        let t322 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t295 * t79 - 3.0 / 10.0 * t21 * t147 * t162 + 3.0 / 10.0 * t21 * t61 * t303 - 3.0 / 20.0 * t21 * t61 * t317);
        let tv2sigma20 = 2.0 * rho[ip] * t322;
        v2sigma2[ip] += tv2sigma20;
    }
}
