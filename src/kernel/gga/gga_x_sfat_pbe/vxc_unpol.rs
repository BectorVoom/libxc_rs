//! GGA_X_SFAT_PBE vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sfat_pbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t17 / t4 * t3;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t27 = M_CBRT6;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = 1.0 / t30;
        let t32 = t31 * t27;
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = t34 * sigma[ip];
        let t36 = rho[ip] * rho[ip];
        let t37 = t19 * t19;
        let t39 = 1.0 / t37 / t36;
        let t43 = 0.804e0 + 0.91464571985215458336e-2 * t39 * t35 * t32;
        let t46 = 0.1804e1 - 0.646416e0 / t43;
        let t49 = 1.0 / t46 * t25 * t24 * t20 * M_PI;
        let t50 = f64::sqrt(t49);
        let t52 = 1.0 / t50 * param_hyb_omega_0;
        let t53 = rho[ip] * t11;
        let t54 = pow_1_3(t53);
        let t55 = 1.0 / t54;
        let t58 = t55 * t33 * t52 / 2.0;
        let t59 = 0.192e1 <= t58;
        let t60 = 0.192e1 < t58;
        let t61 = piecewise3(t60, t58, 0.192e1);
        let t62 = t61 * t61;
        let t63 = t62 * t62;
        let t64 = 1.0 / t63;
        let t66 = t63 * t62;
        let t67 = 1.0 / t66;
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = t69 * t62;
        let t73 = 1.0 / t72;
        let t75 = t69 * t63;
        let t76 = 1.0 / t75;
        let t78 = t69 * t66;
        let t79 = 1.0 / t78;
        let t81 = t69 * t69;
        let t82 = 1.0 / t81;
        let t85 = 1.0 / t81 / t62;
        let t88 = 1.0 / t81 / t63;
        let t91 = 1.0 / t81 / t66;
        let t94 = 1.0 / t81 / t69;
        let t97 = 1.0 / t81 / t72;
        let t100 = 1.0 / t81 / t75;
        let t103 = 1.0 / t81 / t78;
        let t105 = t81 * t81;
        let t106 = 1.0 / t105;
        let t109 = 1.0 / t105 / t62;
        let t112 = 1.0 / t105 / t63;
        let t116 = -t64 / 30.0 + t67 / 70.0 - t70 / 135.0 + t73 / 231.0 - t76 / 364.0 + t79 / 540.0 - t82 / 765.0 + t85 / 1045.0 - t88 / 1386.0 + t91 / 1794.0 - t94 / 2275.0 + t97 / 2835.0 - t100 / 3480.0 + t103 / 4216.0 - t106 / 5049.0 + t109 / 5985.0 - t112 / 7030.0 + 1.0 / t62 / 9.0;
        let t117 = piecewise3(t60, 0.192e1, t58);
        let t118 = f64::atan2(1.0, t117);
        let t119 = t117 * t117;
        let t120 = t119 + 3.0;
        let t121 = 1.0 / t119;
        let t122 = 1.0 + t121;
        let t123 = f64::ln(t122);
        let t125 = -t123 * t120 + 1.0;
        let t128 = t118 + t125 * t117 / 4.0;
        let t132 = piecewise3(t59, t116, 1.0 - 8.0 / 3.0 * t128 * t117);
        let t137 = piecewise3(t2, 0.0, -3.0 / 8.0 * t46 * t132 * t19 * t18);
        let tzk0 = 2.0 * t137;
        zk[ip] += tzk0;
        let t138 = 1.0 / t37;
        let t143 = t63 * t61;
        let t144 = 1.0 / t143;
        let t147 = 1.0 / t50 / t49 * param_hyb_omega_0;
        let t149 = t24 * t20;
        let t150 = t25 * t149;
        let t151 = t150 * t55 * t147;
        let t152 = t46 * t46;
        let t153 = 1.0 / t152;
        let t154 = t43 * t43;
        let t155 = 1.0 / t154;
        let t157 = t27 * t155 * t153;
        let t158 = sigma[ip] * t31;
        let t159 = t36 * rho[ip];
        let t161 = 1.0 / t37 / t159;
        let t167 = 1.0 / t54 / t53;
        let t172 = -0.24765871385369419417e-1 * t161 * t158 * t157 * t151 - t11 * t167 * t33 * t52 / 6.0;
        let t173 = piecewise3(t60, t172, 0.0);
        let t176 = t62 * t61;
        let t177 = t63 * t176;
        let t178 = 1.0 / t177;
        let t181 = t69 * t61;
        let t182 = 1.0 / t181;
        let t185 = t69 * t176;
        let t186 = 1.0 / t185;
        let t189 = t69 * t143;
        let t190 = 1.0 / t189;
        let t193 = t69 * t177;
        let t194 = 1.0 / t193;
        let t198 = 1.0 / t81 / t61;
        let t202 = 1.0 / t81 / t176;
        let t206 = 1.0 / t81 / t143;
        let t210 = 1.0 / t81 / t177;
        let t214 = 1.0 / t81 / t181;
        let t218 = 1.0 / t81 / t185;
        let t222 = 1.0 / t81 / t189;
        let t226 = 1.0 / t81 / t193;
        let t230 = 1.0 / t105 / t61;
        let t234 = 1.0 / t105 / t176;
        let t238 = 1.0 / t105 / t143;
        let t241 = 1.0 / t176;
        let t244 = 2.0 / 15.0 * t173 * t144 - 3.0 / 35.0 * t173 * t178 + 8.0 / 135.0 * t173 * t182 - 10.0 / 231.0 * t173 * t186 + 3.0 / 91.0 * t173 * t190 - 7.0 / 270.0 * t173 * t194 + 16.0 / 765.0 * t173 * t198 - 18.0 / 1045.0 * t173 * t202 + 10.0 / 693.0 * t173 * t206 - 11.0 / 897.0 * t173 * t210 + 24.0 / 2275.0 * t173 * t214 - 26.0 / 2835.0 * t173 * t218 + 7.0 / 870.0 * t173 * t222 - 15.0 / 2108.0 * t173 * t226 + 32.0 / 5049.0 * t173 * t230 - 34.0 / 5985.0 * t173 * t234 + 18.0 / 3515.0 * t173 * t238 - 2.0 / 9.0 * t173 * t241;
        let t245 = piecewise3(t60, 0.0, t172);
        let t248 = 1.0 / t122;
        let t254 = t119 * t117;
        let t255 = 1.0 / t254;
        let t256 = t255 * t120;
        let t257 = t248 * t245;
        let t260 = -2.0 * t123 * t245 * t117 + 2.0 * t257 * t256;
        let t263 = -t248 * t121 * t245 + t125 * t245 / 4.0 + t260 * t117 / 4.0;
        let t267 = piecewise3(t59, t244, -8.0 / 3.0 * t263 * t117 - 8.0 / 3.0 * t128 * t245);
        let t272 = t17 * t3;
        let t274 = 1.0 / t19 / t159;
        let t276 = t132 * t274 * t272;
        let t277 = t27 * t155;
        let t278 = t34 * t158;
        let t279 = t278 * t277;
        let t283 = piecewise3(t2, 0.0, -t46 * t132 * t138 * t18 / 8.0 - 3.0 / 8.0 * t46 * t267 * t19 * t18 + 0.40369036088841097646e-2 * t279 * t276);
        let tvrho0 = 2.0 * t283 * rho[ip] + 2.0 * t137;
        vrho[ip] += tvrho0;
        let t288 = t24 * t20 * t55 * t147;
        let t289 = t153 * t25;
        let t290 = t155 * t289;
        let t294 = 0.92872017695135322815e-2 * t39 * t32 * t290 * t288;
        let t295 = piecewise3(t60, t294, 0.0);
        let t296 = t295 * t144;
        let t298 = t295 * t178;
        let t300 = t295 * t182;
        let t302 = t295 * t186;
        let t304 = t295 * t190;
        let t306 = t295 * t194;
        let t308 = t295 * t198;
        let t310 = t295 * t202;
        let t312 = t295 * t206;
        let t314 = t295 * t210;
        let t316 = t295 * t214;
        let t318 = t295 * t218;
        let t320 = t295 * t222;
        let t322 = t295 * t226;
        let t324 = t295 * t230;
        let t326 = t295 * t234;
        let t328 = t295 * t238;
        let t332 = 2.0 / 15.0 * t296 - 3.0 / 35.0 * t298 + 8.0 / 135.0 * t300 - 10.0 / 231.0 * t302 + 3.0 / 91.0 * t304 - 7.0 / 270.0 * t306 + 16.0 / 765.0 * t308 - 18.0 / 1045.0 * t310 + 10.0 / 693.0 * t312 - 11.0 / 897.0 * t314 + 24.0 / 2275.0 * t316 - 26.0 / 2835.0 * t318 + 7.0 / 870.0 * t320 - 15.0 / 2108.0 * t322 + 32.0 / 5049.0 * t324 - 34.0 / 5985.0 * t326 + 18.0 / 3515.0 * t328 - 2.0 / 9.0 * t295 * t241;
        let t333 = piecewise3(t60, 0.0, t294);
        let t335 = t121 * t333;
        let t341 = t248 * t333;
        let t344 = -2.0 * t123 * t333 * t117 + 2.0 * t341 * t256;
        let t347 = -t248 * t335 + t125 * t333 / 4.0 + t344 * t117 / 4.0;
        let t351 = piecewise3(t59, t332, -8.0 / 3.0 * t347 * t117 - 8.0 / 3.0 * t128 * t333);
        let t357 = 1.0 / t19 / t36;
        let t360 = t34 * t31;
        let t361 = t360 * t277;
        let t365 = piecewise3(t2, 0.0, -3.0 / 8.0 * t46 * t351 * t19 * t18 - 0.15138388533315411618e-2 * t361 * t132 * t357 * t272);
        let tvsigma0 = 2.0 * t365 * rho[ip];
        vsigma[ip] += tvsigma0;
    }
}
