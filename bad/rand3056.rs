
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3056(_: S2, _: S7, _: S7) {}
        
        fn test3056() { foo3056(S4, S6, S3, S8, S2, S5); }
    