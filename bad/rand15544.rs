
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15544(_: S1, _: S4, _: S8, _: S2) {}
        
        fn test15544() { foo15544(S0, S7, S7, S4, S0, S2); }
    