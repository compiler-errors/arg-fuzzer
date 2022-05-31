
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3689(_: S8, _: S5, _: S6, _: S7, _: S3, _: S1) {}
        
        fn test3689() { foo3689(S2, S4, S5, S3, S2, S6, S4); }
    