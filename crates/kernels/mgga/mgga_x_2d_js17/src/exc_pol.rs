//! MGGA_X_2D_JS17 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_2d_js17_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t4 = 1.0 / t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5::<f64>(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = f64::sqrt(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = f64::sqrt(t18);
        let t23 = t22 * t18;
        let t24 = piecewise3::<f64>(t19, t21, t23);
        let t25 = t4 * t24;
        let t26 = M_SQRT2;
        let t27 = f64::sqrt(t5);
        let t28 = t26 * t27;
        let t29 = rho0 * rho0;
        let t30 = t29 * rho0;
        let t31 = 1.0 / t30;
        let t32 = sigma0 * t31;
        let t34 = sigma0 * sigma0;
        let t35 = t29 * t29;
        let t37 = 1.0 / t35 / t29;
        let t40 = 1.0 + 0.41252961249419271031e0 * t32 + 0.63029881920225480858e-3 * t34 * t37;
        let t41 = f64::powf(t40, 1.0 / 15.0);
        let t44 = 1.0 / t29;
        let t48 = 1.0 / M_PI;
        let t51 = 1.0 + 0.27938513438760141227e-1 * t32 + (-0.772e-1 * tau0 * t44 - 0.11596246802930644802e2) * t48 / 4.0;
        let t52 = f64::powf(t40, 1.0 / 5.0);
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t41 + 2.0 / 5.0 * t51 * t53;
        let t57 = t28 * t56;
        let t60 = piecewise3::<f64>(t2, 0.0, -2.0 / 3.0 * t25 * t57);
        let t61 = rho1 <= dens_threshold;
        let t62 = -t15;
        let t64 = piecewise5::<f64>(t13, t10, t9, t14, t62 * t6);
        let t65 = 1.0 + t64;
        let t66 = t65 <= zeta_threshold;
        let t67 = f64::sqrt(t65);
        let t68 = t67 * t65;
        let t69 = piecewise3::<f64>(t66, t21, t68);
        let t70 = t4 * t69;
        let t71 = rho1 * rho1;
        let t72 = t71 * rho1;
        let t73 = 1.0 / t72;
        let t74 = sigma2 * t73;
        let t76 = sigma2 * sigma2;
        let t77 = t71 * t71;
        let t79 = 1.0 / t77 / t71;
        let t82 = 1.0 + 0.41252961249419271031e0 * t74 + 0.63029881920225480858e-3 * t76 * t79;
        let t83 = f64::powf(t82, 1.0 / 15.0);
        let t86 = 1.0 / t71;
        let t92 = 1.0 + 0.27938513438760141227e-1 * t74 + (-0.772e-1 * tau1 * t86 - 0.11596246802930644802e2) * t48 / 4.0;
        let t93 = f64::powf(t82, 1.0 / 5.0);
        let t94 = 1.0 / t93;
        let t97 = 1.0 / t83 + 2.0 / 5.0 * t92 * t94;
        let t98 = t28 * t97;
        let t101 = piecewise3::<f64>(t61, 0.0, -2.0 / 3.0 * t70 * t98);
        let tzk0 = t60 + t101;
        zk[ip] += tzk0;
    }
}
