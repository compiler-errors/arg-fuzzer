
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3032(_: S2, _: S3, _: S4, _: S6, _: S7, _: S8) {}
        
        fn test3032() { foo3032(S2, S2, S4, S4, S6, S8, S1, S5); }
    