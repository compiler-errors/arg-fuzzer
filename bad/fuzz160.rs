
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo160(_: S5, _: S4, _: S4, _: S5) {}
        
        fn test160() { foo160(S4, S5, S6, S8, S4, S8, S3); }
    