
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13688(_: S1, _: S3, _: S5, _: S7, _: S8) {}
        
        fn test13688() { foo13688(S1, S6, S1, S1, S5, S7, S6); }
    