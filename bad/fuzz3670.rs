
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3670(_: S6, _: S5, _: S2, _: S2) {}
        
        fn test3670() { foo3670(S1, S2, S5, S6, S7, S8); }
    