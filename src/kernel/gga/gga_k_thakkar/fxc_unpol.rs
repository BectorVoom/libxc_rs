//! GGA_K_THAKKAR fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_thakkar_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
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
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = t30 * t24;
        let t33 = 1.0 / t21 / rho[ip];
        let t35 = f64::ln(t31 * t33 + f64::sqrt(pow_2(t31 * t33) + 1.0));
        let t36 = t33 * t35;
        let t39 = 1.0 + 0.253e-1 * t31 * t36;
        let t40 = 1.0 / t39;
        let t44 = M_CBRT4;
        let t49 = 2.0 * t44 * t30 * t24 * t33 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t33 * t50;
        let t54 = 1.0 + 0.55e-2 * t26 * t29 * t40 - 0.72e-1 * t31 * t51;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        let t60 = t20 / t21;
        let t64 = t27 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t70 = t39 * t39;
        let t71 = 1.0 / t70;
        let t72 = t29 * t71;
        let t74 = 1.0 / t21 / t27;
        let t75 = t74 * t35;
        let t78 = t26 * t29;
        let t79 = t78 + 1.0;
        let t80 = f64::sqrt(t79);
        let t81 = 1.0 / t80;
        let t82 = t66 * t81;
        let t85 = -0.33733333333333333333e-1 * t31 * t75 - 0.33733333333333333333e-1 * t26 * t82;
        let t89 = t74 * t50;
        let t92 = t49 * t49;
        let t93 = 1.0 / t92;
        let t95 = t66 * t93 * t44;
        let t98 = -0.14666666666666666667e-1 * t26 * t66 * t40 - 0.55e-2 * t26 * t72 * t85 + 0.96e-1 * t31 * t89 - 0.192e0 * t26 * t95;
        let t103 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t106 = t25 * t29;
        let t109 = 1.0 / t30;
        let t110 = t109 * t24;
        let t115 = 0.1265e-1 * t110 * t36 + 0.1265e-1 * t106 * t81;
        let t121 = t93 * t44;
        let t124 = 0.55e-2 * t106 * t40 - 0.55e-2 * t26 * t72 * t115 - 0.36e-1 * t110 * t51 + 0.72e-1 * t106 * t121;
        let t128 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
        let t131 = t20 * t33;
        let t138 = t27 * t27;
        let t140 = 1.0 / t22 / t138;
        let t144 = t66 * t71;
        let t149 = 1.0 / t70 / t39;
        let t150 = t29 * t149;
        let t151 = t85 * t85;
        let t156 = 1.0 / t21 / t64;
        let t157 = t156 * t35;
        let t160 = t140 * t81;
        let t163 = sigma[ip] * sigma[ip];
        let t164 = t163 * t24;
        let t165 = t138 * t64;
        let t167 = 1.0 / t21 / t165;
        let t169 = 1.0 / t80 / t79;
        let t173 = 0.7871111111111111111e-1 * t31 * t157 + 0.16866666666666666667e0 * t26 * t160 - 0.89955555555555555555e-1 * t164 * t167 * t169;
        let t177 = t156 * t50;
        let t181 = t140 * t93 * t44;
        let t184 = t30 * sigma[ip];
        let t185 = t138 * t27;
        let t186 = 1.0 / t185;
        let t189 = 1.0 / t92 / t49;
        let t190 = t44 * t44;
        let t191 = t189 * t190;
        let t194 = 0.53777777777777777779e-1 * t26 * t140 * t40 + 0.29333333333333333334e-1 * t26 * t144 * t85 + 0.11e-1 * t26 * t150 * t151 - 0.55e-2 * t26 * t72 * t173 - 0.224e0 * t31 * t177 + 0.96e0 * t26 * t181 - 0.2048e1 * t184 * t186 * t191;
        let t199 = piecewise3(t2, 0.0, -t7 * t131 * t54 / 30.0 + t7 * t60 * t98 / 5.0 + 3.0 / 20.0 * t7 * t23 * t194);
        let tv2rho20 = 2.0 * rho[ip] * t199 + 4.0 * t103;
        v2rho2[ip] += tv2rho20;
        let t205 = t25 * t66;
        let t208 = t71 * t85;
        let t214 = t149 * t115;
        let t215 = t214 * t85;
        let t223 = 1.0 / t21 / t185;
        let t224 = t24 * t223;
        let t225 = t169 * sigma[ip];
        let t228 = -0.16866666666666666667e-1 * t110 * t75 - 0.506e-1 * t205 * t81 + 0.33733333333333333333e-1 * t224 * t225;
        let t236 = t138 * rho[ip];
        let t237 = 1.0 / t236;
        let t239 = t190 * t30;
        let t242 = -0.14666666666666666667e-1 * t205 * t40 - 0.55e-2 * t106 * t208 + 0.14666666666666666667e-1 * t26 * t144 * t115 + 0.11e-1 * t78 * t215 - 0.55e-2 * t26 * t72 * t228 + 0.48e-1 * t110 * t89 - 0.288e0 * t205 * t121 + 0.768e0 * t237 * t189 * t239;
        let t247 = piecewise3(t2, 0.0, t7 * t60 * t124 / 10.0 + 3.0 / 20.0 * t7 * t23 * t242);
        let tv2rhosigma0 = 2.0 * rho[ip] * t247 + 2.0 * t128;
        v2rhosigma[ip] += tv2rhosigma0;
        let t250 = t71 * t115;
        let t253 = t115 * t115;
        let t257 = 1.0 / t184;
        let t258 = t257 * t24;
        let t261 = 1.0 / sigma[ip];
        let t262 = t261 * t25;
        let t263 = t29 * t81;
        let t267 = 1.0 / t21 / t236;
        let t271 = -0.6325e-2 * t258 * t36 + 0.6325e-2 * t262 * t263 - 0.1265e-1 * t24 * t267 * t169;
        let t278 = t29 * t93 * t44;
        let t281 = 1.0 / t138;
        let t286 = -0.11e-1 * t106 * t250 + 0.11e-1 * t26 * t150 * t253 - 0.55e-2 * t26 * t72 * t271 + 0.18e-1 * t258 * t51 + 0.36e-1 * t262 * t278 - 0.288e0 * t281 * t189 * t190 * t109;
        let t290 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t286);
        let tv2sigma20 = 2.0 * rho[ip] * t290;
        v2sigma2[ip] += tv2sigma20;
    }
}
