use std::fs::File;
use std::io::BufReader;
use rodio::{ OutputStream, Sink};
use winapi::um::winuser::GetAsyncKeyState;
use std::thread;






fn play_voice(num : u8) -> bool {
let mut file : File = File::open("jojo.wav").unwrap();
let (_stream, stream_handle) = OutputStream::try_default().unwrap();
let sink = Sink::try_new(&stream_handle).unwrap();
    match num {
        4=> file = File::open("sounds/D4She/Dimension Hop.mp3").unwrap(),
        3=> file = File::open("sounds/D4She/Pocket Revolver.mp3").unwrap(),
        2=>file = File::open("sounds/D4She/Stand Barrage Finisher.mp3").unwrap(),
        1=>file = File::open("sounds/D4She/Stand Barrage.mp3").unwrap(),
        0=>file = File::open("sounds/D4She/Stand Summon.mp3").unwrap(),
        _=> println!("Passed wrong value to play_voice REPORT THIS TO ley#0001 ASAP"),
    }

    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
true
}

unsafe fn create_key(v_key : i32 , kd : &bool, num: u8, cd : &bool) -> Vec<bool>{
    
    let mut keydown =*kd;
    let mut cooldown = *cd;
    if cooldown == true {
        return Vec::from([keydown,cooldown])
    }
  
    if GetAsyncKeyState(v_key) & 0x01 != 0 && !keydown{
        keydown = true;
        
        cooldown = thread::spawn(move || -> bool {
            
          if !cooldown {
            
            play_voice(num);
           
          }
           
           
            
            
            
            
       true
     }).join().unwrap();
    
    

} else  if GetAsyncKeyState(v_key)  == 0 && keydown{
    println!("Entered");
    keydown = false;
}

Vec::from([keydown,cooldown])


}




fn main() {
    
   
 
    let mut stand_summon : bool = false;
    let mut barrage_buffer = Vec::from([false,false]);
    let mut barrage_finish_buffer = Vec::from([false,false]);
    let mut t_key_special_buffer = Vec::from([false,false]);
    let mut z_key_special_buffer = Vec::from([false,false]);
    let mut barrage_timer = std::time::Instant::now();
    let mut barrage_fin_timer = std::time::Instant::now();
    let mut t_timer = std::time::Instant::now();
    let mut z_timer = std::time::Instant::now();
    let mut started = false;
    let mut fin_started = false;
    let mut t_started = false;
    let mut z_started = false;

    
loop{
    unsafe{
       
        
        //e = 0x45
            //Summon
            if GetAsyncKeyState(0x51) & 0x01 != 0 {
               
                if stand_summon == false{
                    if play_voice(0) == true{
                    stand_summon = true;
                    }
                } else {
                    stand_summon = false;
                }

                
        }


        if stand_summon == true{


            
            //Barage
            barrage_buffer  =     
                      create_key(0x45,&barrage_buffer[0],1,&barrage_buffer[1]);
                      
                
           
            
            // Barage Finisher
            
            barrage_finish_buffer =  create_key(0x52, &barrage_finish_buffer[0],2,&barrage_finish_buffer[1]);
          


            // T Key
            t_key_special_buffer =  create_key(0x54, & t_key_special_buffer[0],3,& t_key_special_buffer[1]);
 

           
            //Z Ability
            z_key_special_buffer =  create_key( 0x5A, & z_key_special_buffer[0],4,& z_key_special_buffer[1]);
            

            //Barage CD
            if barrage_buffer[1] == true && started == false{
                barrage_timer = std::time::Instant::now();
               started = true;      
            }
            if barrage_timer.elapsed().as_secs() == 8{
                barrage_buffer[1] = false;
                started = false;

               }

            
            // barrage_fin_timer CD

            if barrage_finish_buffer[1] == true && fin_started == false{
                barrage_fin_timer = std::time::Instant::now();
               fin_started = true;      
            }
            if barrage_fin_timer.elapsed().as_secs() == 7{
                barrage_finish_buffer[1] = false;
                fin_started = false;

               }

               //T Key
               if t_key_special_buffer[1] == true && t_started == false{
                t_timer = std::time::Instant::now();
               t_started = true;      
            }
            if t_timer.elapsed().as_millis() == 5860{
                t_key_special_buffer[1] = false;
                t_started = false;

               }

                //Z Key
                if z_key_special_buffer[1] == true && z_started == false{
                    z_timer = std::time::Instant::now();
                   z_started = true;      
                }
                if z_timer.elapsed().as_secs() == 57{
                    z_key_special_buffer[1] = false;
                    z_started = false;
    
                   }

        }
    }
}
}
       




            
            
        
        
    
    
  









