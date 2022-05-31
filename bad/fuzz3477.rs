
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3477(_: S2, _: S5, _: S4) {}
        
        fn test3477() { foo3477(S7, S8, S1, S2, S3, S7, S3); }
    