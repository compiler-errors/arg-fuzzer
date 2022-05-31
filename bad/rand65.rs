
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo65(_: S1, _: S5, _: S7) {}
        
        fn test65() { foo65(S5, S6, S1, S3); }
    