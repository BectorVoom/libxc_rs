//! GGA_X_GG99 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_gg99.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_1_4, pow_2};
use crate::math::special::{xc_dilogarithm};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_gg99_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t8 = t4 / t5 / M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t17 = pow_1_3(t13);
        let t19 = piecewise3(t13 <= zeta_threshold, t15 * zeta_threshold, t17 * t13);
        let t20 = pow_1_3(rho[ip]);
        let t21 = t19 * t20;
        let t22 = t8 * t21;
        let t23 = M_PI * M_PI;
        let t24 = f64::sqrt(sigma[ip]);
        let t25 = M_CBRT2;
        let t26 = t24 * t25;
        let t28 = 1.0 / t20 / rho[ip];
        let t29 = t26 * t28;
        let t30 = M_CBRT4;
        let t31 = f64::sqrt(3.0);
        let t32 = t23 * M_PI;
        let t33 = t31 * t32;
        let t34 = pow_1_3(t33);
        let t35 = t30 * t34;
        let t36 = t29 < t35;
        let t37 = pow_1_4(3.0);
        let t38 = M_SQRT2;
        let t39 = t37 * t38;
        let t40 = f64::sqrt(M_PI);
        let t42 = 1.0 / t40 / M_PI;
        let t43 = t39 * t42;
        let t44 = t35 - 0.1e-9;
        let t45 = t44 < t29;
        let t46 = piecewise3(t45, t44, t29);
        let t47 = t46 * t46;
        let t49 = t23 * t23;
        let t50 = t49 * t23;
        let t52 = t47 * t47;
        let t53 = t52 * t47;
        let t54 = 48.0 * t50 - t53;
        let t55 = f64::sqrt(t54);
        let t56 = 4.0 * t33 + t55;
        let t57 = pow_1_3(t56);
        let t58 = t57 * t57;
        let t59 = t47 + t58;
        let t60 = f64::sqrt(t59);
        let t62 = f64::powf(t56, 1.0 / 6.0);
        let t63 = 1.0 / t62;
        let t67 = f64::ln(t43 * t46 * t60 * t63 / 4.0 + f64::sqrt(pow_2(t43 * t46 * t60 * t63 / 4.0) + 1.0));
        let t68 = 1.0 / M_PI;
        let t69 = t35 + 0.1e-9;
        let t70 = t69 < t29;
        let t71 = piecewise3(t70, t29, t69);
        let t72 = t71 * t71;
        let t73 = t72 * t71;
        let t74 = t73 * t31;
        let t76 = t72 * t72;
        let t77 = t76 * t72;
        let t80 = 3.0 / t50 * t77 - 144.0;
        let t81 = f64::sqrt(t80);
        let t83 = f64::atan(t81 / 12.0);
        let t84 = t83 / 3.0;
        let t85 = f64::cos(t84);
        let t86 = t68 * t85;
        let t87 = t74 * t86;
        let t88 = f64::sqrt(t87);
        let t91 = f64::ln(t68 * t88 / 2.0 + f64::sqrt(pow_2(t68 * t88 / 2.0) + 1.0));
        let t92 = piecewise3(t36, t67, t91);
        let t94 = f64::exp(-2.0 * t92);
        let t95 = 1.0 + t94;
        let t96 = f64::ln(t95);
        let t99 = xc_dilogarithm(-t94);
        let t101 = -12.0 * t92 * t96 + t23 + 12.0 * t99;
        let t102 = 1.0 / t92;
        let t104 = 1.0 / f64::cosh(t92);
        let t105 = pow_1_3(t104);
        let t106 = t105 * t105;
        let t107 = 1.0 / t106;
        let t108 = pow_1_3(t68);
        let t109 = 1.0 / t108;
        let t111 = t107 * t109 * t30;
        let t112 = t101 * t102 * t111;
        let t115 = piecewise3(t2, 0.0, -t22 * t112 / 24.0);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
        let t116 = t20 * t20;
        let t117 = 1.0 / t116;
        let t118 = t19 * t117;
        let t119 = t8 * t118;
        let t122 = rho[ip] * rho[ip];
        let t124 = 1.0 / t20 / t122;
        let t126 = 4.0 / 3.0 * t26 * t124;
        let t127 = piecewise3(t45, 0.0, -t126);
        let t132 = 1.0 / t60;
        let t133 = t46 * t132;
        let t135 = 1.0 / t57;
        let t136 = 1.0 / t55;
        let t137 = t135 * t136;
        let t138 = t52 * t46;
        let t139 = t138 * t127;
        let t142 = 2.0 * t127 * t46 - 2.0 * t137 * t139;
        let t143 = t63 * t142;
        let t147 = t42 * t53;
        let t148 = t39 * t147;
        let t150 = 1.0 / t62 / t56;
        let t151 = t60 * t150;
        let t152 = t136 * t127;
        let t156 = t43 * t127 * t60 * t63 / 4.0 + t43 * t133 * t143 / 8.0 + t148 * t151 * t152 / 8.0;
        let t157 = 1.0 / t32;
        let t158 = t31 * t157;
        let t163 = 2.0 * t135 * t158 * t47 * t59 + 16.0;
        let t164 = f64::sqrt(t163);
        let t165 = 1.0 / t164;
        let t169 = t68 / t88;
        let t170 = t72 * t31;
        let t171 = piecewise3(t70, -t126, 0.0);
        let t175 = t170 * t68;
        let t176 = 1.0 / t81;
        let t177 = t176 * t171;
        let t178 = f64::sin(t84);
        let t179 = t177 * t178;
        let t182 = 3.0 * t170 * t171 * t86 - 12.0 * t175 * t179;
        let t183 = t157 * t85;
        let t185 = t183 * t74 + 4.0;
        let t186 = f64::sqrt(t185);
        let t187 = 1.0 / t186;
        let t188 = t182 * t187;
        let t191 = piecewise3(t36, 4.0 * t156 * t165, t169 * t188 / 2.0);
        let t195 = 1.0 / t95;
        let t196 = t94 * t195;
        let t199 = 24.0 * t191 * t196 * t92 + 12.0 * t191 * t96;
        let t201 = t199 * t102 * t111;
        let t205 = t8 * t21 * t101;
        let t206 = t92 * t92;
        let t207 = 1.0 / t206;
        let t208 = t207 * t107;
        let t209 = t109 * t30;
        let t211 = t208 * t209 * t191;
        let t215 = t102 * t107 * t109;
        let t216 = t30 * t191;
        let t217 = f64::tanh(t92);
        let t219 = t215 * t216 * t217;
        let t223 = piecewise3(t2, 0.0, -t119 * t112 / 72.0 - t22 * t201 / 24.0 + t205 * t211 / 24.0 - t205 * t219 / 36.0);
        let tvrho0 = 2.0 * rho[ip] * t223 + 2.0 * t115;
        vrho[ip] += tvrho0;
        let t227 = 1.0 / t24 * t25;
        let t229 = t227 * t28 / 2.0;
        let t230 = piecewise3(t45, 0.0, t229);
        let t236 = t138 * t230;
        let t239 = -2.0 * t137 * t236 + 2.0 * t230 * t46;
        let t240 = t63 * t239;
        let t244 = t136 * t230;
        let t248 = t43 * t230 * t60 * t63 / 4.0 + t43 * t133 * t240 / 8.0 + t148 * t151 * t244 / 8.0;
        let t251 = piecewise3(t70, t229, 0.0);
        let t252 = t86 * t251;
        let t255 = t176 * t251;
        let t256 = t255 * t178;
        let t259 = 3.0 * t170 * t252 - 12.0 * t175 * t256;
        let t260 = t259 * t187;
        let t263 = piecewise3(t36, 4.0 * t248 * t165, t169 * t260 / 2.0);
        let t266 = t92 * t263;
        let t269 = 24.0 * t196 * t266 + 12.0 * t263 * t96;
        let t271 = t269 * t102 * t111;
        let t275 = t208 * t209 * t263;
        let t278 = t30 * t263;
        let t280 = t215 * t278 * t217;
        let t284 = piecewise3(t2, 0.0, -t22 * t271 / 24.0 + t205 * t275 / 24.0 - t205 * t280 / 36.0);
        let tvsigma0 = 2.0 * rho[ip] * t284;
        vsigma[ip] += tvsigma0;
    }
}
