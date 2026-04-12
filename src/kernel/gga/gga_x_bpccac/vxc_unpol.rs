//! GGA_X_BPCCAC vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bpccac.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_bpccac_vxc_unpol(
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
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = f64::sqrt(sigma[ip]);
        let t21 = M_CBRT2;
        let t24 = 1.0 / t18 / rho[ip];
        let t25 = t20 * t21 * t24;
        let t27 = f64::exp(-t25 + 19.0);
        let t28 = 1.0 + t27;
        let t29 = 1.0 / t28;
        let t30 = 1.0 - t29;
        let t31 = M_CBRT6;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t36 = t31 * t35;
        let t37 = t21 * t21;
        let t38 = sigma[ip] * t37;
        let t39 = rho[ip] * rho[ip];
        let t40 = t18 * t18;
        let t42 = 1.0 / t40 / t39;
        let t43 = t38 * t42;
        let t44 = t36 * t43;
        let t46 = 0.1227e1 + 0.91464571985215458336e-2 * t44;
        let t49 = 0.2227e1 - 0.1505529e1 / t46;
        let t52 = f64::exp(-25.0 / 6.0 * t44);
        let t55 = (0.2743e0 - 0.1508e0 * t52) * t31;
        let t56 = t55 * t35;
        let t59 = t31 * t31;
        let t61 = 1.0 / t33 / t32;
        let t62 = t59 * t61;
        let t63 = sigma[ip] * sigma[ip];
        let t64 = t63 * t21;
        let t65 = t39 * t39;
        let t66 = t65 * rho[ip];
        let t68 = 1.0 / t18 / t66;
        let t71 = 0.13888888888888888889e-4 * t62 * t64 * t68;
        let t72 = t56 * t43 / 24.0 - t71;
        let t74 = t59 / t33;
        let t75 = t74 * t20;
        let t76 = t21 * t24;
        let t79 = f64::ln(0.64963333333333333333e0 * t74 * t25 + f64::sqrt(pow_2(0.64963333333333333333e0 * t74 * t25) + 1.0));
        let t80 = t76 * t79;
        let t83 = 1.0 + 0.16370833333333333333e-1 * t75 * t80 + t71;
        let t84 = 1.0 / t83;
        let t86 = t72 * t84 + 1.0;
        let t88 = t29 * t86 + t30 * t49;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t88);
        let tzk0 = 2.0 * t92;
        zk[ip] += tzk0;
        let t94 = t17 / t40;
        let t98 = t28 * t28;
        let t99 = 1.0 / t98;
        let t100 = t99 * t20;
        let t101 = t100 * t21;
        let t103 = 1.0 / t18 / t39;
        let t105 = t103 * t27 * t49;
        let t108 = t46 * t46;
        let t109 = 1.0 / t108;
        let t111 = t30 * t109 * t31;
        let t112 = t35 * sigma[ip];
        let t113 = t39 * rho[ip];
        let t115 = 1.0 / t40 / t113;
        let t116 = t37 * t115;
        let t120 = t99 * t86;
        let t121 = t120 * t20;
        let t122 = t21 * t103;
        let t123 = t122 * t27;
        let t126 = t62 * t63;
        let t127 = t65 * t39;
        let t129 = 1.0 / t18 / t127;
        let t130 = t21 * t129;
        let t131 = t130 * t52;
        let t139 = 0.74074074074074074075e-4 * t62 * t64 * t129;
        let t140 = -0.13962962962962962963e0 * t126 * t131 - t56 * t38 * t115 / 9.0 + t139;
        let t142 = t83 * t83;
        let t143 = 1.0 / t142;
        let t144 = t72 * t143;
        let t145 = t122 * t79;
        let t148 = t36 * sigma[ip];
        let t150 = 0.25321408066666666666e1 * t44 + 1.0;
        let t151 = f64::sqrt(t150);
        let t152 = 1.0 / t151;
        let t153 = t116 * t152;
        let t156 = -0.21827777777777777777e-1 * t75 * t145 - 0.85080312222222222219e-1 * t148 * t153 - t139;
        let t158 = t140 * t84 - t144 * t156;
        let t160 = 4.0 / 3.0 * t101 * t105 - 0.36720684159021185007e-1 * t111 * t112 * t116 - 4.0 / 3.0 * t121 * t123 + t29 * t158;
        let t165 = piecewise3(t2, 0.0, -t6 * t94 * t88 / 8.0 - 3.0 / 8.0 * t6 * t19 * t160);
        let tvrho0 = 2.0 * rho[ip] * t165 + 2.0 * t92;
        vrho[ip] += tvrho0;
        let t168 = 1.0 / t20;
        let t169 = t99 * t168;
        let t170 = t169 * t21;
        let t172 = t24 * t27 * t49;
        let t175 = t35 * t37;
        let t176 = t175 * t42;
        let t179 = t120 * t168;
        let t180 = t76 * t27;
        let t183 = t62 * t21;
        let t190 = sigma[ip] * t21;
        let t193 = 0.27777777777777777778e-4 * t62 * t190 * t68;
        let t194 = 0.52361111111111111112e-1 * t183 * t68 * t52 * sigma[ip] + t55 * t176 / 24.0 - t193;
        let t196 = t74 * t168;
        let t199 = t37 * t42;
        let t200 = t199 * t152;
        let t203 = 0.81854166666666666665e-2 * t196 * t80 + 0.31905117083333333333e-1 * t36 * t200 + t193;
        let t205 = -t144 * t203 + t194 * t84;
        let t207 = -t170 * t172 / 2.0 + 0.13770256559632944377e-1 * t111 * t176 + t179 * t180 / 2.0 + t29 * t205;
        let t211 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t207);
        let tvsigma0 = 2.0 * rho[ip] * t211;
        vsigma[ip] += tvsigma0;
    }
}
