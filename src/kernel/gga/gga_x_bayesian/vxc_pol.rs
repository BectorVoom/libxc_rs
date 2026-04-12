//! GGA_X_BAYESIAN vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_bayesian_vxc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t28 * t28;
        let t41 = 1.0 / t30;
        let t42 = t40 * t41;
        let t43 = f64::sqrt(sigma0);
        let t49 = 1.0 + t42 * t43 / t36 / rho0 / 12.0;
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t39 * t51;
        let t57 = 0.1926e0 + 0.79008333333333333333e-1 * t33 * sigma0 * t39 * t51;
        let t58 = t52 * t57;
        let t61 = 0.10008e1 + t34 * t58 / 24.0;
        let t65 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t61);
        let t66 = rho1 <= dens_threshold;
        let t67 = -t16;
        let t69 = piecewise5(t14, t11, t10, t15, t67 * t7);
        let t70 = 1.0 + t69;
        let t71 = t70 <= zeta_threshold;
        let t72 = pow_1_3(t70);
        let t74 = piecewise3(t71, t22, t72 * t70);
        let t75 = t74 * t26;
        let t76 = t33 * sigma2;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = f64::sqrt(sigma2);
        let t88 = 1.0 + t42 * t82 / t78 / rho1 / 12.0;
        let t89 = t88 * t88;
        let t90 = 1.0 / t89;
        let t91 = t81 * t90;
        let t96 = 0.1926e0 + 0.79008333333333333333e-1 * t33 * sigma2 * t81 * t90;
        let t97 = t91 * t96;
        let t100 = 0.10008e1 + t76 * t97 / 24.0;
        let t104 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t75 * t100);
        let tzk0 = t65 + t104;
        zk[ip] += tzk0;
        let t105 = t6 * t6;
        let t106 = 1.0 / t105;
        let t107 = t16 * t106;
        let t109 = piecewise5(t10, 0.0, t14, 0.0, t7 - t107);
        let t112 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t109);
        let t113 = t112 * t26;
        let t117 = t26 * t26;
        let t118 = 1.0 / t117;
        let t119 = t25 * t118;
        let t122 = t5 * t119 * t61 / 8.0;
        let t123 = t35 * rho0;
        let t125 = 1.0 / t37 / t123;
        let t126 = t125 * t51;
        let t127 = t126 * t57;
        let t130 = 1.0 / t29;
        let t131 = t43 * sigma0;
        let t132 = t130 * t131;
        let t133 = t35 * t35;
        let t134 = t133 * rho0;
        let t135 = 1.0 / t134;
        let t137 = 1.0 / t50 / t49;
        let t138 = t135 * t137;
        let t148 = -0.21068888888888888889e0 * t33 * sigma0 * t125 * t51 + 0.10534444444444444444e0 * t132 * t138;
        let t149 = t52 * t148;
        let t152 = -t34 * t127 / 9.0 + t132 * t138 * t57 / 18.0 + t34 * t149 / 24.0;
        let t157 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t113 * t61 - t122 - 3.0 / 8.0 * t5 * t27 * t152);
        let t158 = t67 * t106;
        let t160 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t158);
        let t163 = piecewise3(t71, 0.0, 4.0 / 3.0 * t72 * t160);
        let t164 = t163 * t26;
        let t168 = t74 * t118;
        let t171 = t5 * t168 * t100 / 8.0;
        let t173 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t164 * t100 - t171);
        let tvrho0 = t65 + t104 + t6 * (t157 + t173);
        vrho[ip * 2] += tvrho0;
        let t177 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t107);
        let t180 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t177);
        let t181 = t180 * t26;
        let t186 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t181 * t61 - t122);
        let t188 = piecewise5(t14, 0.0, t10, 0.0, t7 - t158);
        let t191 = piecewise3(t71, 0.0, 4.0 / 3.0 * t72 * t188);
        let t192 = t191 * t26;
        let t196 = t77 * rho1;
        let t198 = 1.0 / t79 / t196;
        let t199 = t198 * t90;
        let t200 = t199 * t96;
        let t203 = t82 * sigma2;
        let t204 = t130 * t203;
        let t205 = t77 * t77;
        let t206 = t205 * rho1;
        let t207 = 1.0 / t206;
        let t209 = 1.0 / t89 / t88;
        let t210 = t207 * t209;
        let t220 = -0.21068888888888888889e0 * t33 * sigma2 * t198 * t90 + 0.10534444444444444444e0 * t204 * t210;
        let t221 = t91 * t220;
        let t224 = -t76 * t200 / 9.0 + t204 * t210 * t96 / 18.0 + t76 * t221 / 24.0;
        let t229 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t192 * t100 - t171 - 3.0 / 8.0 * t5 * t75 * t224);
        let tvrho1 = t65 + t104 + t6 * (t186 + t229);
        vrho[ip * 2 + 1] += tvrho1;
        let t234 = t130 * t43;
        let t235 = 1.0 / t133;
        let t236 = t235 * t137;
        let t244 = 0.79008333333333333333e-1 * t33 * t52 - 0.39504166666666666666e-1 * t234 * t236;
        let t245 = t52 * t244;
        let t248 = t33 * t58 / 24.0 - t234 * t236 * t57 / 48.0 + t34 * t245 / 24.0;
        let t252 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t248);
        let tvsigma0 = t6 * t252;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t255 = t130 * t82;
        let t256 = 1.0 / t205;
        let t257 = t256 * t209;
        let t265 = 0.79008333333333333333e-1 * t33 * t91 - 0.39504166666666666666e-1 * t255 * t257;
        let t266 = t91 * t265;
        let t269 = t33 * t97 / 24.0 - t255 * t257 * t96 / 48.0 + t76 * t266 / 24.0;
        let t273 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t75 * t269);
        let tvsigma2 = t6 * t273;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
