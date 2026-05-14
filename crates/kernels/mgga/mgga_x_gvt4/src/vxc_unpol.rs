//! MGGA_X_GVT4 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_gvt4_vxc_unpol(
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
        let t4 = M_CBRTPI;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t22 = sigma[ip] * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t29 = tau[ip] * t21;
        let t31 = 1.0 / t24 / rho[ip];
        let t32 = t29 * t31;
        let t34 = M_CBRT6;
        let t35 = t34 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = t35 * t38;
        let t41 = 1.0 + 0.186726e-2 * t27 + 0.373452e-2 * t32 - 0.1120356e-2 * t39;
        let t47 = -0.3556788e-2 * t27 + 0.12500652e-1 * t32 - 0.37501956e-2 * t39;
        let t48 = t41 * t41;
        let t49 = 1.0 / t48;
        let t51 = sigma[ip] * sigma[ip];
        let t52 = t51 * t20;
        let t53 = t23 * t23;
        let t54 = t53 * rho[ip];
        let t56 = 1.0 / t18 / t54;
        let t61 = 2.0 * t32 - 3.0 / 5.0 * t39;
        let t65 = t61 * t61;
        let t67 = -0.4709036e-4 * t52 * t56 - 0.1282732e-3 * t22 * t26 * t61 + 0.3574822e-3 * t65;
        let t68 = t48 * t41;
        let t69 = 1.0 / t68;
        let t73 = pow_1_3(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683e0 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3(t3, 0.0, t19 * t77 / 4.0);
        let tzk0 = 2.0 * t80;
        zk[ip] += tzk0;
        let t82 = t17 / t24;
        let t85 = t23 * rho[ip];
        let t87 = 1.0 / t24 / t85;
        let t88 = t22 * t87;
        let t90 = t29 * t26;
        let t92 = -0.497936e-2 * t88 - 0.62242e-2 * t90;
        let t97 = 0.9484768e-2 * t88 - 0.2083442e-1 * t90;
        let t99 = t47 * t69;
        let t102 = t53 * t23;
        let t104 = 1.0 / t18 / t102;
        let t110 = sigma[ip] * t20;
        let t114 = t61 * tau[ip];
        let t115 = t21 * t26;
        let t118 = 0.25114858666666666667e-3 * t52 * t104 + 0.34206186666666666667e-3 * t22 * t87 * t61 + 0.85515466666666666667e-3 * t110 * t56 * tau[ip] - 0.23832146666666666667e-2 * t114 * t115;
        let t120 = t48 * t48;
        let t121 = 1.0 / t120;
        let t122 = t67 * t121;
        let t127 = (0.9800683e0 * t49 * t92 + t97 * t49 - 2.0 * t99 * t92 + t118 * t69 - 3.0 * t122 * t92) * t74 * t76;
        let t131 = piecewise3(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t80;
        vrho[ip] += tvrho0;
        let t134 = t49 * t21;
        let t135 = t134 * t26;
        let t137 = t99 * t115;
        let t139 = t110 * t56;
        let t141 = t115 * t61;
        let t143 = -0.9418072e-4 * t139 - 0.1282732e-3 * t141;
        let t145 = t122 * t115;
        let t149 = (-0.1726745666142e-2 * t135 - 0.373452e-2 * t137 + t143 * t69 - 0.560178e-2 * t145) * t74 * t76;
        let t152 = piecewise3(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.5130928e-3 * t110 * t160 + 0.14299288e-2 * t61 * t21 * t31;
        let t172 = (0.16160736667716e-1 * t134 * t31 - 0.746904e-2 * t99 * t156 + t166 * t69 - 0.1120356e-1 * t122 * t156) * t74 * t76;
        let t175 = piecewise3(t3, 0.0, t19 * t172 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t175;
        vtau[ip] += tvtau0;
    }
}
