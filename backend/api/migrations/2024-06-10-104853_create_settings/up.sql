CREATE TABLE settings (
    key   VARCHAR(255)  NOT NULL PRIMARY KEY,
    value VARCHAR(1024) NOT NULL
);


INSERT INTO settings (key, value)
VALUES ('app_name', 'ULPN');
INSERT INTO settings (key, value)
VALUES ('navigation_drawer_open', 'Open navigation drawer.');
INSERT INTO settings (key, value)
VALUES ('navigation_drawer_close', 'Close navigation drawer.');
INSERT INTO settings (key, value)
VALUES ('nav_header_title', 'De Unges Liv på Nettet');
INSERT INTO settings (key, value)
VALUES ('nav_header_subtitle', 'Kontakt: ####@####.###');
INSERT INTO settings (key, value)
VALUES ('nav_header_desc', 'Navigation header');
INSERT INTO settings (key, value)
VALUES ('action_login', 'Log ind');
INSERT INTO settings (key, value)

VALUES ('home_title', 'Jeg har ret til privatliv!');
INSERT INTO settings (key, value)
VALUES ('home_desc', 'No Text.');

INSERT INTO settings (key, value)
VALUES ('menu_home', 'Hjem');
INSERT INTO settings (key, value)
VALUES ('menu_slideshow', 'Slideshow');
INSERT INTO settings (key, value)
VALUES ('logout', 'Log ud');
INSERT INTO settings (key, value)
VALUES ('login_with_google', 'Log ind med Google');
INSERT INTO settings (key, value)
VALUES ('google_sign_in_failed', 'Der skete en fejl...');
INSERT INTO settings (key, value)
VALUES ('home_info_title', 'Hvorfor gør vi dette her?');
INSERT INTO settings (key, value)
VALUES ('home_info_desc', 
'''
- Sikre Børn og unges sikkerhed online.
                            
Give børn og unge en platform, hvor de kan italesætte hvad de oplever online.
Give forældrer mulighed for at spare med andrer forældrer omkring deres erfaringer.
Anonymitet til en vis grænse (se brugerpolikiken).
En platform der belyser muligheder og risiciene online.
''');

