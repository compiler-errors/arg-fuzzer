
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8190(_: S7, _: S6, _: S5, _: S1, _: S7, _: S7) {}
        
        fn test8190() { foo8190(S1, S2, S3, S4, S5, S6, S7, S8); }
    