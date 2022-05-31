
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12589(_: S5, _: S4, _: S5) {}
        
        fn test12589() { foo12589(S7, S7, S4, S2); }
    