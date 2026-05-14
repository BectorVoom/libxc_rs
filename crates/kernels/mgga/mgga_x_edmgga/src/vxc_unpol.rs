//! MGGA_X_EDMGGA vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_edmgga_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT4;
        let t22 = t4 * t4;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t27 = t21 * t22 * t25 / 9.0;
        let t28 = 1.0 - t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = tau[ip] * t30;
        let t32 = t19 * t19;
        let t34 = 1.0 / t32 / rho[ip];
        let t36 = sigma[ip] * t30;
        let t37 = rho[ip] * rho[ip];
        let t39 = 1.0 / t32 / t37;
        let t42 = lapl[ip] * t30;
        let t46 = M_CBRT6;
        let t48 = t25 * t25;
        let t49 = 1.0 / t48;
        let t50 = (t31 * t34 - t36 * t39 / 8.0 - t42 * t34 / 4.0) * t46 * t49;
        let t51 = 5.0 / 9.0 * t50;
        let t52 = -t51 < -0.14205545454545454545e5;
        let t53 = 0.39111111111111111111e0 * t50;
        let t55 = 0.0 < 0.70414204545454545455e0 - t53;
        let t57 = piecewise3(t55, -0.14204545454545454545e-3, 0.704e0 - t53);
        let t60 = t57 * t57;
        let t61 = t60 * t57;
        let t62 = 1.0 / t61;
        let t65 = 1.0 - t51;
        let t66 = t65 * t65;
        let t68 = 1.0 + 0.495616e0 * t66;
        let t69 = f64::sqrt(t68);
        let t71 = piecewise3(t52, -1.0 / t57 / 2.0 + t62 / 8.0, 0.704e0 - t53 + t69);
        let t72 = t28 * t71;
        let t73 = f64::sqrt(30.0);
        let t74 = t28 * t73;
        let t75 = f64::sqrt(t71);
        let t76 = t28 * t28;
        let t81 = 0.60184783083548636238e0 * t76 - 0.206514e-1;
        let t82 = t71 - 1.0;
        let t86 = f64::ln(0.39102932048925120047e0 / t76 / t28 * t73 * t81 * t82 + f64::sqrt(pow_2(0.39102932048925120047e0 / t76 / t28 * t73 * t81 * t82) + 1.0));
        let t90 = 1.0 + 0.14163895778062926267e0 * t74 * t75 * t86;
        let t91 = 1.0 / t90;
        let t93 = t72 * t91 + t27;
        let t97 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t93);
        let tzk0 = 2.0 * t97;
        zk[ip] += tzk0;
        let t99 = t18 / t32;
        let t103 = 1.0 / t60;
        let t106 = t37 * rho[ip];
        let t108 = 1.0 / t32 / t106;
        let t113 = -5.0 / 3.0 * t31 * t39 + t36 * t108 / 3.0 + 5.0 / 12.0 * t42 * t39;
        let t115 = t113 * t46 * t49;
        let t116 = 0.39111111111111111111e0 * t115;
        let t117 = piecewise3(t55, 0.0, -t116);
        let t120 = t60 * t60;
        let t121 = 1.0 / t120;
        let t125 = 1.0 / t69;
        let t126 = t125 * t65;
        let t130 = piecewise3(t52, t103 * t117 / 2.0 - 3.0 / 8.0 * t121 * t117, -t116 - 0.27534222222222222222e0 * t126 * t115);
        let t131 = t28 * t130;
        let t133 = t90 * t90;
        let t134 = 1.0 / t133;
        let t135 = 1.0 / t75;
        let t136 = t135 * t86;
        let t140 = 1.0 / t76;
        let t141 = t140 * t75;
        let t142 = t81 * t130;
        let t143 = t76 * t76;
        let t144 = t143 * t76;
        let t146 = t81 * t81;
        let t148 = t82 * t82;
        let t151 = 0.4587117884468565861e1 / t144 * t146 * t148 + 1.0;
        let t152 = f64::sqrt(t151);
        let t153 = 1.0 / t152;
        let t157 = 0.70819478890314631335e-1 * t74 * t136 * t130 + 0.16615495624729559964e1 * t141 * t142 * t153;
        let t158 = t134 * t157;
        let t160 = t131 * t91 - t72 * t158;
        let t165 = piecewise3(t3, 0.0, -t7 * t99 * t93 / 8.0 - 3.0 / 8.0 * t7 * t20 * t160);
        let tvrho0 = 2.0 * rho[ip] * t165 + 2.0 * t97;
        vrho[ip] += tvrho0;
        let t168 = t30 * t39;
        let t169 = t46 * t49;
        let t170 = t168 * t169;
        let t171 = 0.48888888888888888889e-1 * t170;
        let t172 = piecewise3(t55, 0.0, t171);
        let t175 = t121 * t172;
        let t178 = t126 * t30;
        let t180 = t39 * t46 * t49;
        let t181 = t178 * t180;
        let t184 = piecewise3(t52, t103 * t172 / 2.0 - 3.0 / 8.0 * t175, t171 + 0.34417777777777777778e-1 * t181);
        let t185 = t28 * t184;
        let t194 = 0.70819478890314631335e-1 * t74 * t136 * t184 + 0.16615495624729559964e1 * t141 * t81 * t184 * t153;
        let t195 = t134 * t194;
        let t197 = t185 * t91 - t72 * t195;
        let t201 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t197);
        let tvsigma0 = 2.0 * rho[ip] * t201;
        vsigma[ip] += tvsigma0;
        let t203 = t30 * t34;
        let t204 = t203 * t169;
        let t205 = 0.97777777777777777778e-1 * t204;
        let t206 = piecewise3(t55, 0.0, t205);
        let t209 = t121 * t206;
        let t213 = t34 * t46 * t49;
        let t214 = t178 * t213;
        let t217 = piecewise3(t52, t103 * t206 / 2.0 - 3.0 / 8.0 * t209, t205 + 0.68835555555555555555e-1 * t214);
        let t218 = t28 * t217;
        let t227 = 0.70819478890314631335e-1 * t74 * t136 * t217 + 0.16615495624729559964e1 * t141 * t81 * t217 * t153;
        let t228 = t134 * t227;
        let t230 = t218 * t91 - t72 * t228;
        let t234 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t230);
        let tvlapl0 = 2.0 * rho[ip] * t234;
        vlapl[ip] += tvlapl0;
        let t236 = 0.39111111111111111111e0 * t204;
        let t237 = piecewise3(t55, 0.0, -t236);
        let t240 = t121 * t237;
        let t245 = piecewise3(t52, t103 * t237 / 2.0 - 3.0 / 8.0 * t240, -t236 - 0.27534222222222222222e0 * t214);
        let t246 = t28 * t245;
        let t255 = 0.70819478890314631335e-1 * t74 * t136 * t245 + 0.16615495624729559964e1 * t141 * t81 * t245 * t153;
        let t256 = t134 * t255;
        let t258 = t246 * t91 - t72 * t256;
        let t262 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t258);
        let tvtau0 = 2.0 * rho[ip] * t262;
        vtau[ip] += tvtau0;
    }
}
