
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3108(_: S1, _: S3, _: S5, _: S8) {}
        
        fn test3108() { foo3108(S2, S6, S1, S4, S7); }
    