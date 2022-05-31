
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3640(_: S2, _: S8, _: S2, _: S7) {}
        
        fn test3640() { foo3640(S1, S5, S1, S5, S6, S8, S1, S1); }
    